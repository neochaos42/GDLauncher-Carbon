use super::{discovery::Discovery, java_checker::JavaChecker};
use crate::domain::java::{
    JavaArch, JavaComponent, JavaComponentType, JavaVersion, SystemJavaProfileName,
};
use carbon_repos::db::{read_filters::StringFilter, PrismaClient};
use std::{path::PathBuf, sync::Arc};
use strum::IntoEnumIterator;
use tracing::{info, trace, warn};

#[tracing::instrument(level = "trace", skip(db))]
async fn get_java_component_from_db(
    db: &PrismaClient,
    path: String,
) -> anyhow::Result<Option<carbon_repos::db::java::Data>> {
    let res = db
        .java()
        .find_unique(carbon_repos::db::java::UniqueWhereParam::PathEquals(path))
        .exec()
        .await?;

    Ok(res)
}

#[tracing::instrument(level = "trace", skip(db))]
pub async fn upsert_java_component_to_db(
    db: &Arc<PrismaClient>,
    java_component: JavaComponent,
) -> anyhow::Result<String> {
    let already_existing_component =
        get_java_component_from_db(db, java_component.path.clone()).await?;

    let already_existing_component = already_existing_component
        .map(|data| {
            (
                JavaComponent::try_from(data.clone()),
                data.is_valid,
                data.id,
            )
        })
        .and_then(|res| {
            let resp = res.0.ok();

            match resp {
                Some(val) => Some((val, res.1, res.2)),
                None => None,
            }
        });

    if let Some((component, is_valid, id)) = already_existing_component {
        if component == java_component {
            db.java()
                .update(
                    carbon_repos::db::java::id::equals(id.clone()),
                    vec![carbon_repos::db::java::is_valid::set(true)],
                )
                .exec()
                .await?;

            return Ok(id);
        } else if component.version.major == java_component.version.major {
            db.java()
                .update(
                    carbon_repos::db::java::id::equals(id.clone()),
                    vec![
                        carbon_repos::db::java::full_version::set(
                            java_component.version.to_string(),
                        ),
                        carbon_repos::db::java::arch::set(java_component.arch.to_string()),
                        carbon_repos::db::java::os::set(java_component.os.to_string()),
                        carbon_repos::db::java::vendor::set(java_component.vendor),
                        carbon_repos::db::java::is_valid::set(true),
                    ],
                )
                .exec()
                .await?;

            return Ok(id);
        } else {
            anyhow::bail!(
                "Java component with same path but different major version already exists"
            );
        }
    } else {
        let res = db
            .java()
            .create(
                java_component.path,
                java_component.version.major as i32,
                java_component.version.to_string(),
                java_component._type.to_string(),
                java_component.os.to_string(),
                java_component.arch.to_string(),
                java_component.vendor,
                vec![],
            )
            .exec()
            .await?;

        Ok(res.id)
    }
}

#[tracing::instrument(level = "trace", skip(db))]
async fn update_java_component_in_db_to_invalid(
    db: &Arc<PrismaClient>,
    path: String,
) -> anyhow::Result<()> {
    db.java()
        .update(
            carbon_repos::db::java::UniqueWhereParam::PathEquals(path),
            vec![carbon_repos::db::java::SetParam::SetIsValid(false)],
        )
        .exec()
        .await?;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn scan_and_sync_local<T, G>(
    db: &Arc<PrismaClient>,
    discovery: &T,
    java_checker: &G,
) -> anyhow::Result<()>
where
    T: Discovery,
    G: JavaChecker,
{
    let local_javas = discovery.find_java_paths().await;
    let java_profiles = db
        .java_profile()
        .find_many(vec![])
        .with(carbon_repos::db::java_profile::java::fetch())
        .exec()
        .await?;

    for local_java in &local_javas {
        trace!("Analyzing local java: {:?}", local_java);

        let resolved_java_path = match dunce::canonicalize(local_java) {
            Ok(canonical_path) => canonical_path,
            Err(err) => {
                tracing::warn!("Error resolving canonical java path: {}", err);
                local_java.to_path_buf()
            }
        };

        // Verify whether the java is valid
        let java_bin_info = java_checker
            .get_bin_info(&resolved_java_path, JavaComponentType::Local)
            .await;

        let db_entry =
            get_java_component_from_db(db, resolved_java_path.to_string_lossy().to_string())
                .await?;

        if let Some(db_entry) = &db_entry {
            if JavaComponentType::try_from(&*db_entry.r#type)? != JavaComponentType::Local {
                continue;
            }
        }

        let is_java_used_in_profile = java_profiles.iter().any(|profile| {
            let Some(java) = profile.java.as_ref() else {
                return false;
            };
            let Some(java) = java.as_ref() else {
                return false;
            };
            let java_path = java.path.clone();
            java_path == resolved_java_path.display().to_string()
        });

        match (java_bin_info, db_entry) {
            // If it is valid, check whether it's in the DB
            (Ok(java_component), Some(db_entry)) => {
                trace!("Java is valid: {:?}", java_component);
                upsert_java_component_to_db(db, java_component).await?;
            }
            (Ok(java_component), None) => {
                trace!("Java is valid: {:?}", java_component);
                upsert_java_component_to_db(db, java_component).await?;
            }
            // If it isn't valid, check whether it's in the DB
            (Err(err), db_entry) => {
                trace!("Java is invalid due to: {:?}", err);

                // If it is in the db, update it to invalid
                if db_entry.is_some() {
                    if is_java_used_in_profile {
                        update_java_component_in_db_to_invalid(
                            db,
                            resolved_java_path.display().to_string(),
                        )
                        .await?;
                    } else {
                        db.java()
                            .delete(carbon_repos::db::java::UniqueWhereParam::PathEquals(
                                resolved_java_path.display().to_string(),
                            ))
                            .exec()
                            .await?;
                    }
                }
            }
        }
    }

    // Cleanup unscanned local javas (if they are not default)
    let local_javas_from_db = db
        .java()
        .find_many(vec![carbon_repos::db::java::r#type::equals(
            JavaComponentType::Local.to_string(),
        )])
        .exec()
        .await?;

    for local_java_from_db in local_javas_from_db {
        trace!(
            "Checking if java {} has been scanned",
            local_java_from_db.path
        );
        let has_been_scanned = local_javas
            .iter()
            .any(|local_java| local_java_from_db.path == local_java.display().to_string());

        if has_been_scanned {
            continue;
        }

        let is_used_in_profile = java_profiles
            .iter()
            .filter_map(|profile| {
                let Some(java) = profile.java.as_ref() else {
                    return None;
                };
                let Some(java) = java else {
                    return None;
                };
                Some(java.path.clone())
            })
            .any(|java_profile_path| local_java_from_db.path == java_profile_path);

        if is_used_in_profile {
            update_java_component_in_db_to_invalid(db, local_java_from_db.path).await?;
        } else {
            db.java()
                .delete(carbon_repos::db::java::UniqueWhereParam::PathEquals(
                    local_java_from_db.path,
                ))
                .exec()
                .await?;
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn scan_and_sync_custom<G>(db: &Arc<PrismaClient>, java_checker: &G) -> anyhow::Result<()>
where
    G: JavaChecker,
{
    let custom_javas = db
        .java()
        .find_many(vec![carbon_repos::db::java::WhereParam::Type(
            StringFilter::Equals(JavaComponentType::Custom.to_string()),
        )])
        .exec()
        .await?;

    for custom_java in custom_javas {
        let java_bin_info = java_checker
            .get_bin_info(
                &PathBuf::from(custom_java.path.clone()),
                JavaComponentType::Custom,
            )
            .await;

        if java_bin_info.is_err() {
            update_java_component_in_db_to_invalid(db, custom_java.path).await?;
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn scan_and_sync_managed<T, G>(
    db: &Arc<PrismaClient>,
    discovery: &T,
    java_checker: &G,
) -> anyhow::Result<()>
where
    T: Discovery,
    G: JavaChecker,
{
    let managed_javas = db
        .java()
        .find_many(vec![carbon_repos::db::java::r#type::equals(
            JavaComponentType::Managed.to_string(),
        )])
        .exec()
        .await?;

    let java_profiles = db
        .java_profile()
        .find_many(vec![])
        .with(carbon_repos::db::java_profile::java::fetch())
        .exec()
        .await?;

    for managed_java in &managed_javas {
        let java_bin_info = java_checker
            .get_bin_info(
                &PathBuf::from(managed_java.path.clone()),
                JavaComponentType::Managed,
            )
            .await;

        let is_java_used_in_profile = java_profiles.iter().any(|profile| {
            let Some(java) = profile.java.as_ref() else {
                return false;
            };
            let Some(java) = java.as_ref() else {
                return false;
            };
            let java_path = java.path.clone();
            java_path == managed_java.path
        });

        info!(
            "java {} is used in profile: {}",
            managed_java.path, is_java_used_in_profile
        );

        match (java_bin_info, managed_java.is_valid) {
            (Ok(java_component), true) => {}
            (Ok(java_component), false) => {
                upsert_java_component_to_db(db, java_component).await?;
            }
            (Err(_), true) => {
                if is_java_used_in_profile {
                    update_java_component_in_db_to_invalid(db, managed_java.path.clone()).await?;
                } else {
                    db.java()
                        .delete(carbon_repos::db::java::path::equals(
                            managed_java.path.clone(),
                        ))
                        .exec()
                        .await?;
                }
            }
            (Err(_), false) => {
                if !is_java_used_in_profile {
                    db.java()
                        .delete(carbon_repos::db::java::UniqueWhereParam::PathEquals(
                            managed_java.path.clone(),
                        ))
                        .exec()
                        .await?;
                }
            }
        }
    }

    let javas_on_disk = discovery.find_managed_java_paths().await;

    for java_path in javas_on_disk.iter().filter(|path| {
        !managed_javas
            .iter()
            .any(|java| java.path == path.to_string_lossy().to_string())
    }) {
        let java_bin_info = java_checker
            .get_bin_info(&java_path, JavaComponentType::Managed)
            .await;

        if let Ok(java_component) = java_bin_info {
            upsert_java_component_to_db(db, java_component).await?;
        }
    }

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn sync_system_java_profiles(db: &Arc<PrismaClient>) -> anyhow::Result<()> {
    let all_javas = db.java().find_many(vec![]).exec().await?;

    let is32bit = std::env::consts::ARCH == "x86" || std::env::consts::ARCH == "arm";

    for profile in SystemJavaProfileName::iter() {
        trace!("Syncing system java profile: {}", profile.to_string());
        let java_in_profile = db
            .java_profile()
            .find_unique(carbon_repos::db::java_profile::name::equals(
                profile.to_string(),
            ))
            .exec()
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Java system profile {} not found in DB",
                    profile.to_string()
                )
            })?
            .java_id;

        if java_in_profile.is_some() {
            trace!(
                "Java system profile {} already has a java",
                profile.to_string()
            );
            continue;
        }

        // Scan for a compatible java
        for java in all_javas.iter() {
            trace!("Checking java {}", java.path);
            if !java.is_valid {
                warn!("Java {} is invalid, skipping", java.path);
                continue;
            }

            let java_version = JavaVersion::try_from(java.full_version.as_str())?;
            let java_arch = JavaArch::try_from(java.arch.as_str())?;

            let is_arch_allowed = match java_arch {
                JavaArch::X86_32 | JavaArch::Arm32 => is32bit,
                _ => true,
            };

            if profile.is_java_version_compatible(&java_version) && is_arch_allowed {
                trace!(
                    "Java {} is compatible with profile {}",
                    java.path,
                    profile.to_string()
                );
                db.java_profile()
                    .update(
                        carbon_repos::db::java_profile::name::equals(profile.to_string()),
                        vec![carbon_repos::db::java_profile::java::connect(
                            carbon_repos::db::java::id::equals(java.id.clone()),
                        )],
                    )
                    .exec()
                    .await?;
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use tracing::info;

    use crate::{
        domain::java::{
            JavaArch, JavaComponent, JavaComponentType, JavaOs, JavaVersion, SystemJavaProfileName,
        },
        managers::java::{
            discovery::MockDiscovery,
            java_checker::{MockJavaChecker, MockJavaCheckerInvalid},
            scan_and_sync::{
                scan_and_sync_custom, scan_and_sync_local, scan_and_sync_managed,
                sync_system_java_profiles, upsert_java_component_to_db,
            },
            JavaManager,
        },
        setup_managers_for_test,
    };

    #[tokio::test]
    async fn test_add_component_to_db() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;

        let java_path = "/usr/bin/java2".to_string();

        let java_component = JavaComponent {
            path: java_path.clone(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };
        let java_components = db.java().find_many(vec![]).exec().await.unwrap();
        assert_eq!(java_components.len(), 0);

        upsert_java_component_to_db(db, java_component.clone())
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();
        assert_eq!(java_components.len(), 1);
        assert_eq!(java_components[0].path, "/usr/bin/java2");
        assert!(java_components[0].is_valid);

        db.java()
            .update(
                carbon_repos::db::java::path::equals(java_path.clone()),
                vec![carbon_repos::db::java::is_valid::set(false)],
            )
            .exec()
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();
        assert_eq!(java_components.len(), 1);
        assert!(!java_components[0].is_valid);

        upsert_java_component_to_db(db, java_component)
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();
        assert_eq!(java_components.len(), 1);
        assert!(java_components[0].is_valid);

        let almost_equal_java_component = JavaComponent {
            path: java_path.clone(),
            version: JavaVersion::from_major(9), // different version
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        let result = upsert_java_component_to_db(db, almost_equal_java_component).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_scan_and_sync_local() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;

        let discovery = &MockDiscovery;
        let java_checker = &MockJavaChecker;
        // Insert one already existing path (/usr/bin/java) and one that should not exist anymore, hence removed (/usr/bin/java2)

        let component_to_remove = JavaComponent {
            path: "/java1".to_string(),
            version: JavaVersion::from_major(19),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };
        upsert_java_component_to_db(db, component_to_remove)
            .await
            .unwrap();

        let component_to_keep = JavaComponent {
            path: "/java4".to_string(),
            version: JavaVersion::from_major(19),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        upsert_java_component_to_db(db, component_to_keep)
            .await
            .unwrap();

        scan_and_sync_local(db, discovery, java_checker)
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        println!("{:?}", java_components);

        assert_eq!(java_components.len(), 3);
    }

    #[tokio::test]
    /// This test is to make sure that if a java is invalid and not used in any profile, it will be removed
    /// If it's used in a profile, it will be set as invalid
    async fn test_scan_and_sync_local_broken_javas() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;
        let discovery = &MockDiscovery;
        let java_checker = &MockJavaCheckerInvalid;

        let component_to_add = JavaComponent {
            path: "/usr/bin/java".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        let component_to_add_still_used = JavaComponent {
            path: "/usr/bin/java1".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        upsert_java_component_to_db(db, component_to_add)
            .await
            .unwrap();
        let java_id = upsert_java_component_to_db(db, component_to_add_still_used)
            .await
            .unwrap();

        db.java_profile()
            .update(
                carbon_repos::db::java_profile::name::equals(
                    SystemJavaProfileName::Legacy.to_string(),
                ),
                vec![carbon_repos::db::java_profile::java::connect(
                    carbon_repos::db::java::id::equals(java_id),
                )],
            )
            .exec()
            .await
            .unwrap();

        scan_and_sync_local(db, discovery, java_checker)
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        assert_eq!(java_components.len(), 1);

        assert_eq!(java_components[0].path, "/usr/bin/java1");
        assert!(!java_components[0].is_valid);
    }
    #[tokio::test]
    async fn test_scan_and_sync_managed_broken_javas() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;
        let java_checker = &MockJavaCheckerInvalid;
        let discovery = &MockDiscovery;

        let component_to_add = JavaComponent {
            path: "/my/managed/path".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Managed,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };
        let component_to_add_still_used = JavaComponent {
            path: "/my/managed/path1".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Managed,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        upsert_java_component_to_db(db, component_to_add)
            .await
            .unwrap();
        let java_id = upsert_java_component_to_db(db, component_to_add_still_used)
            .await
            .unwrap();

        db.java_profile()
            .update(
                carbon_repos::db::java_profile::name::equals(
                    SystemJavaProfileName::Legacy.to_string(),
                ),
                vec![carbon_repos::db::java_profile::java::connect(
                    carbon_repos::db::java::id::equals(java_id),
                )],
            )
            .exec()
            .await
            .unwrap();

        scan_and_sync_managed(db, discovery, java_checker)
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        assert_eq!(java_components.len(), 1);

        assert_eq!(java_components[0].path, "/my/managed/path1");
        assert!(!java_components[0].is_valid);
    }
    #[tokio::test]
    async fn test_scan_and_sync_custom_broken_javas() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;
        let java_checker = &MockJavaCheckerInvalid;

        let component_to_add = JavaComponent {
            path: "/my/custom/path".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Custom,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };
        let component_to_add_still_used = JavaComponent {
            path: "/my/custom/path1".to_string(),
            version: JavaVersion::from_major(8),
            _type: JavaComponentType::Custom,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        upsert_java_component_to_db(db, component_to_add)
            .await
            .unwrap();
        let java_id = upsert_java_component_to_db(db, component_to_add_still_used)
            .await
            .unwrap();

        db.java_profile()
            .update(
                carbon_repos::db::java_profile::name::equals(
                    SystemJavaProfileName::Legacy.to_string(),
                ),
                vec![carbon_repos::db::java_profile::java::connect(
                    carbon_repos::db::java::id::equals(java_id),
                )],
            )
            .exec()
            .await
            .unwrap();

        scan_and_sync_custom(db, java_checker).await.unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        assert_eq!(java_components.len(), 2);

        for java_component in java_components {
            assert!(!java_component.is_valid);
        }
    }

    #[tokio::test]
    async fn test_scan_and_sync_managed_on_disk_but_not_on_database() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;
        let discovery = &MockDiscovery;
        let java_checker = &MockJavaChecker;

        scan_and_sync_managed(db, discovery, java_checker)
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        assert_eq!(java_components.len(), 3);
        for java_component in java_components {
            assert!(java_component.is_valid);
        }
    }

    #[tokio::test]
    async fn test_sync_system_java_profiles_with_profiles() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;

        JavaManager::ensure_profiles_in_db(db).await.unwrap();

        // manually set one of the profiles to non-system to make sure it gets updated to system
        db.java_profile()
            .update(
                carbon_repos::db::java_profile::name::equals(
                    SystemJavaProfileName::Legacy.to_string(),
                ),
                vec![carbon_repos::db::java_profile::is_system_profile::set(
                    false,
                )],
            )
            .exec()
            .await
            .unwrap();

        db.java()
            .create_many(vec![
                (
                    "my_path1".to_string(),
                    8,
                    "1.8.0_282".to_string(),
                    "local".to_string(),
                    "linux".to_string(),
                    "x86_64".to_string(),
                    "Azul Systems, Inc.".to_string(),
                    vec![],
                ),
                (
                    "my_path2".to_string(),
                    17,
                    "17.0.1".to_string(),
                    "local".to_string(),
                    "linux".to_string(),
                    "x86_64".to_string(),
                    "Azul Systems, Inc.".to_string(),
                    vec![],
                ),
                (
                    "my_path3".to_string(),
                    14,
                    "14.0.1".to_string(),
                    "local".to_string(),
                    "linux".to_string(),
                    "x86_64".to_string(),
                    "Azul Systems, Inc.".to_string(),
                    vec![carbon_repos::db::java::SetParam::SetIsValid(false)],
                ),
            ])
            .exec()
            .await
            .unwrap();

        JavaManager::ensure_profiles_in_db(db).await.unwrap();
        sync_system_java_profiles(db).await.unwrap();

        let all_profiles = db.java_profile().find_many(vec![]).exec().await.unwrap();
        assert!(all_profiles.iter().all(|profile| profile.is_system_profile));

        // Expect 8 and 17 to be there, but not 14 since it's invalid and 16 because not provided
        let legacy_profile = db
            .java_profile()
            .find_unique(
                carbon_repos::db::java_profile::UniqueWhereParam::NameEquals(
                    SystemJavaProfileName::Legacy.to_string(),
                ),
            )
            .with(carbon_repos::db::java_profile::java::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        info!("{:?}", legacy_profile);

        assert!(legacy_profile.java.flatten().is_some());

        let alpha_profile = db
            .java_profile()
            .find_unique(
                carbon_repos::db::java_profile::UniqueWhereParam::NameEquals(
                    SystemJavaProfileName::Alpha.to_string(),
                ),
            )
            .with(carbon_repos::db::java_profile::java::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        assert!(alpha_profile.java.flatten().is_none());

        let beta_profile = db
            .java_profile()
            .find_unique(
                carbon_repos::db::java_profile::UniqueWhereParam::NameEquals(
                    SystemJavaProfileName::Beta.to_string(),
                ),
            )
            .with(carbon_repos::db::java_profile::java::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        assert!(beta_profile.java.flatten().is_some());

        let gamma_profile = db
            .java_profile()
            .find_unique(
                carbon_repos::db::java_profile::UniqueWhereParam::NameEquals(
                    SystemJavaProfileName::Gamma.to_string(),
                ),
            )
            .with(carbon_repos::db::java_profile::java::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        assert!(gamma_profile.java.flatten().is_some());

        let minecraft_exe_profile = db
            .java_profile()
            .find_unique(
                carbon_repos::db::java_profile::UniqueWhereParam::NameEquals(
                    SystemJavaProfileName::MinecraftJavaExe.to_string(),
                ),
            )
            .with(carbon_repos::db::java_profile::java::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        assert!(minecraft_exe_profile.java.flatten().is_none());
    }

    #[tokio::test]
    async fn test_upsert_java_component_to_db_different_java_configuration() {
        let app = setup_managers_for_test().await;
        let db = &app.prisma_client;

        let discovery = &MockDiscovery;
        let java_checker = &MockJavaChecker;

        let old_component = JavaComponent {
            path: "/java1".to_string(),
            version: JavaVersion::from_major(19),
            _type: JavaComponentType::Local,
            arch: JavaArch::X86_32,
            os: JavaOs::Linux,
            vendor: "Azul Systems, Inc.".to_string(),
        };

        upsert_java_component_to_db(db, old_component)
            .await
            .unwrap();

        let new_component = JavaComponent {
            path: "/java1".to_string(),
            version: JavaVersion::from_major(19),
            _type: JavaComponentType::Local,
            arch: JavaArch::Arm64,
            os: JavaOs::Windows,
            vendor: "Azul Systems, Inc. New".to_string(),
        };

        scan_and_sync_local(db, discovery, java_checker)
            .await
            .unwrap();

        upsert_java_component_to_db(db, new_component.clone())
            .await
            .unwrap();

        let java_components = db.java().find_many(vec![]).exec().await.unwrap();

        assert_eq!(java_components.len(), 3);

        let java_component = JavaComponent::try_from(java_components[0].clone()).unwrap();

        assert_eq!(java_component, new_component);
    }
}
