use crate::{
    api::translation::Translation,
    domain::{
        instance::{
            info::{GameVersion, ModLoaderType},
            ExportEntry, InstanceId,
        },
        vtask::VisualTaskId,
    },
    managers::{
        instance::{InstanceType, InvalidInstanceIdError},
        modplatforms::modrinth::convert_standard_version_to_mr_version,
        vtask::{TaskState, VisualTask},
        AppInner,
    },
};
use anyhow::anyhow;
use carbon_platforms::modrinth::version::{
    Hashes, ModpackIndex, ModrinthFile, ModrinthGame, ModrinthPackDependencies,
};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::Arc};
use tokio::sync::mpsc;

use carbon_repos::db::{mod_file_cache as fcdb, mod_metadata as metadb};

use super::ZipMode;

pub async fn export_modrinth(
    app: Arc<AppInner>,
    instance_id: InstanceId,
    save_path: PathBuf,
    self_contained_addons_bundling: bool,
    mut filter: ExportEntry,
) -> anyhow::Result<VisualTaskId> {
    let instance_manager = app.instance_manager();
    let instances = instance_manager.instances.read().await;
    let instance = instances
        .get(&instance_id)
        .ok_or(InvalidInstanceIdError(instance_id))?;

    let basepath = app
        .settings_manager()
        .runtime_path
        .get_instances()
        .get_instance_path(&instance.shortpath)
        .get_data_path();

    let InstanceType::Valid(data) = &instance.type_ else {
        return Err(anyhow!("Instance {instance_id} is not in a valid state"));
    };

    let config = data.config.clone();

    drop(instances);

    let Some(version) = config.game_configuration.version else {
        return Err(anyhow!(
            "Instance {instance_id}'s game version is not known so it cannot be exported"
        ));
    };

    let GameVersion::Standard(version) = version else {
        return Err(anyhow!(
            "Instance {instance_id} has a custom game version file so it cannot be exported"
        ));
    };

    let vtask = VisualTask::new(Translation::InstanceExport);
    let vtask_id = app.task_manager().spawn_task(&vtask).await;

    tokio::spawn(async move {
        let try_result: anyhow::Result<_> = async {
            let mut mods = Vec::new();

            let t_calc_size = vtask.subtask(Translation::InstanceExportCalculateSize);
            t_calc_size.set_weight(0.0);
            let t_create_bundle = vtask.subtask(Translation::InstanceExportCreatingBundle);

            vtask
                .edit(|data| data.state = TaskState::KnownProgress)
                .await;

            if !self_contained_addons_bundling {
                let mods_filter = filter.0.get_mut("mods");
                if let Some(mods_filter) = mods_filter {
                    let t_scan = vtask.subtask(Translation::InstanceExportScanningMods);
                    t_calc_size.set_weight(0.5);
                    t_scan.start_opaque();

                    if mods_filter.is_none() {
                        let mut modsdir_entries = HashMap::new();

                        let mut dir = tokio::fs::read_dir(basepath.join("mods")).await?;
                        while let Some(next) = dir.next_entry().await? {
                            let name = next.file_name();
                            let Some(name) = name.to_str() else { continue };
                            modsdir_entries.insert(name.to_string(), None);
                        }

                        *mods_filter = Some(ExportEntry(modsdir_entries));
                    }

                    let mods_filter = mods_filter.as_mut().map(|v| &mut v.0).unwrap();

                    app.meta_cache_manager()
                        .override_caching_and_wait(instance_id, false, true)
                        .await?;

                    let mods2 = app
                        .prisma_client
                        .mod_file_cache()
                        .find_many(vec![fcdb::instance_id::equals(*instance_id)])
                        .with(fcdb::metadata::fetch().with(metadb::modrinth::fetch()))
                        .exec()
                        .await?
                        .into_iter()
                        .filter_map(|m| {
                            let Some(metadata) = m.metadata else {
                                return None;
                            };

                            let Some(Some(modrinth)) = metadata.modrinth else {
                                return None;
                            };

                            match mods_filter.remove(&m.filename) {
                                Some(_) => Some((
                                    m.filename.clone(),
                                    m.filesize,
                                    metadata.sha_512,
                                    metadata.sha_1,
                                    modrinth.file_url,
                                )),
                                None => None,
                            }
                        });

                    mods.extend(mods2);
                    t_scan.complete_opaque();
                }
            }

            t_calc_size.start_opaque();

            let mut file_count = 0;
            super::zip_excluding(
                ZipMode::<File, ()>::Count(&mut file_count),
                &basepath,
                "overrides",
                &filter,
            )?;

            t_calc_size.complete_opaque();
            t_create_bundle.update_items(0, file_count);

            let manifest = ModpackIndex {
                format_version: 1,
                game: ModrinthGame::Minecraft,
                version_id: String::new(),
                name: config.name,
                summary: None,
                files: mods
                    .into_iter()
                    .map(
                        |(file_name, file_size, sha512, sha1, file_url)| ModrinthFile {
                            path: format!("mods/{file_name}"),
                            hashes: Hashes {
                                sha512: hex::encode(sha512),
                                sha1: hex::encode(sha1),
                                others: HashMap::new(),
                            },
                            env: None,
                            downloads: vec![file_url],
                            file_size: file_size as u32,
                        },
                    )
                    .collect(),
                dependencies: convert_standard_version_to_mr_version(version),
            };

            let tmpfile = app
                .settings_manager()
                .runtime_path
                .get_temp()
                .maketmpfile()
                .await?;

            let send_path = tmpfile.to_path_buf();
            let (notify_tx, mut notify_rx) = mpsc::channel::<()>(1);

            let ziptask = tokio::task::spawn_blocking(move || {
                let mut zip = zip::ZipWriter::new(File::create(send_path)?);
                let options = zip::write::FileOptions::<()>::default();
                zip.start_file("modrinth.index.json", options)?;
                zip.write_all(&serde_json::to_vec_pretty(&manifest)?)?;

                super::zip_excluding(
                    ZipMode::Create(&mut zip, options, notify_tx),
                    &basepath,
                    "overrides",
                    &filter,
                )?;

                zip.finish()?;
                Ok::<_, anyhow::Error>(())
            });

            tokio::select! {
                r = ziptask => r??,
                _ = async {
                    let mut counter = 0;

                    loop {
                        if notify_rx.recv().await.is_some() {
                            counter += 1;
                            t_create_bundle.update_items(counter, file_count);
                        } else {
                            futures::future::pending().await
                        }
                    }
                } => {},
            }

            tmpfile.try_rename_or_move(save_path).await?;

            t_create_bundle.complete_items();

            Ok(())
        }
        .await;

        if let Err(e) = try_result {
            vtask.fail(e).await;
        }
    });

    Ok(vtask_id)
}

#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::Read,
        sync::Arc,
    };

    use flowtest::flowtest;
    use tracing_test::traced_test;
    use zip::ZipArchive;

    use crate::{
        domain::instance::{info, ExportEntry, InstanceId},
        managers::instance::{export::ExportTarget, InstanceVersionSource},
    };

    #[traced_test]
    #[test]
    #[flowtest]
    fn _setup() -> anyhow::Result<(
        Arc<tokio::runtime::Runtime>,
        Arc<crate::TestEnv>,
        InstanceId,
    )> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let rt = Arc::new(rt);

        rt.block_on(async {
            let app = Arc::new(crate::setup_managers_for_test().await);

            let default_group_id = app.instance_manager().get_default_group().await?;
            let instance_id = app
                .instance_manager()
                .create_instance(
                    default_group_id,
                    String::from("test"),
                    false,
                    InstanceVersionSource::Version(info::GameVersion::Standard(
                        info::StandardVersion {
                            release: String::from("1.16.5"),
                            modloaders: HashSet::from([info::ModLoader {
                                type_: info::ModLoaderType::Forge,
                                version: String::from("36.2.34"),
                            }]),
                        },
                    )),
                    String::new(),
                )
                .await?;

            let task = app
                .instance_manager()
                .install_modrinth_mod(
                    instance_id,
                    String::from("fPetb5Kh"),
                    String::from("o0SCfsMe"),
                    false,
                    None,
                )
                .await?;

            app.task_manager().wait_with_log(task).await?;

            Ok((rt.clone(), app, instance_id))
        })
    }

    async fn run_export(
        app: &Arc<crate::TestEnv>,
        instance_id: InstanceId,
        filename: &str,
        export_entry: ExportEntry,
        self_contained_addons_bundling: bool,
    ) -> anyhow::Result<()> {
        let target_file = app
            .settings_manager()
            .runtime_path
            .get_root()
            .to_path()
            .join(filename);

        let task = app
            .instance_manager()
            .export_manager()
            .export_instance(
                instance_id,
                ExportTarget::Modrinth,
                target_file.clone(),
                self_contained_addons_bundling,
                export_entry,
            )
            .await?;

        app.task_manager().wait_with_log(task).await?;

        Ok(())
    }

    async fn check_export(
        app: &Arc<crate::TestEnv>,
        filename: &str,
        check: impl FnOnce(String, ZipArchive<File>) -> anyhow::Result<()> + Send + 'static,
    ) -> anyhow::Result<()> {
        let target_file = app
            .settings_manager()
            .runtime_path
            .get_root()
            .to_path()
            .join(filename);

        tokio::task::spawn_blocking(|| {
            let mut zip = ZipArchive::new(File::open(target_file)?)?;

            let mut file = zip.by_name("modrinth.index.json")?;
            let mut manifest = String::new();
            file.read_to_string(&mut manifest)?;
            drop(file);

            check(manifest, zip)
        })
        .await?
    }

    #[traced_test]
    #[test]
    #[flowtest(_setup: (rt, app, instance_id))]
    fn export_with_folder_linked() -> anyhow::Result<()> {
        rt.block_on(async {
            run_export(
                &app,
                instance_id,
                "folder_linked.zip",
                ExportEntry(HashMap::from([(String::from("mods"), None)])),
                false,
            )
            .await?;

            check_export(&app, "folder_linked.zip", |manifest, mut zip| {
                crate::assert_eq_display!(
                    manifest,
                    r#"{
  "formatVersion": 1,
  "game": "minecraft",
  "versionId": "",
  "name": "test",
  "summary": null,
  "files": [
    {
      "path": "mods/NaturesCompass-1.16.5-1.9.1-forge.jar",
      "hashes": {
        "sha512": "bc99c1abb320f84ad7670f35649386855e877d8cce3aaeb12654107e4cdd52acb8475a2a66e6cb5f419dc8cc4d1ecf4c3f6d521e51ee9f1525d1403007e2c0b2",
        "sha1": "38c37c257dcdcf47d5b363eb3e39eebc645b7be4"
      },
      "env": null,
      "downloads": [
        "https://cdn.modrinth.com/data/fPetb5Kh/versions/o0SCfsMe/NaturesCompass-1.16.5-1.9.1-forge.jar"
      ],
      "fileSize": 203573
    }
  ],
  "dependencies": {
    "minecraft": "1.16.5",
    "forge": "36.2.34"
  }
}"#
                );

                assert!(zip.by_name("overrides/mods").is_err());
                Ok(())
            })
            .await?;

            Ok(())
        })
    }

    #[traced_test]
    #[test]
    #[flowtest(_setup: (rt, app, instance_id))]
    fn export_with_folder_unlinked() -> anyhow::Result<()> {
        rt.block_on(async {
            run_export(
                &app,
                instance_id,
                "folder_unlinked.zip",
                ExportEntry(HashMap::from([(String::from("mods"), None)])),
                true,
            )
            .await?;

            check_export(&app, "folder_unlinked.zip", |manifest, mut zip| {
                crate::assert_eq_display!(
                    manifest,
                    r#"{
  "formatVersion": 1,
  "game": "minecraft",
  "versionId": "",
  "name": "test",
  "summary": null,
  "files": [],
  "dependencies": {
    "minecraft": "1.16.5",
    "forge": "36.2.34"
  }
}"#
                );

                assert!(zip
                    .by_name("overrides/mods/NaturesCompass-1.16.5-1.9.1-forge.jar")
                    .is_ok());
                Ok(())
            })
            .await?;

            Ok(())
        })
    }

    #[traced_test]
    #[test]
    #[flowtest(_setup: (rt, app, instance_id))]
    fn export_without_folder_linked() -> anyhow::Result<()> {
        rt.block_on(async {
            run_export(
                &app,
                instance_id,
                "nofolder_linked.zip",
                ExportEntry(HashMap::from([])),
                false,
            )
            .await?;

            check_export(&app, "nofolder_linked.zip", |manifest, mut zip| {
                crate::assert_eq_display!(
                    manifest,
                    r#"{
  "formatVersion": 1,
  "game": "minecraft",
  "versionId": "",
  "name": "test",
  "summary": null,
  "files": [],
  "dependencies": {
    "minecraft": "1.16.5",
    "forge": "36.2.34"
  }
}"#
                );

                assert!(zip.by_name("overrides/mods").is_err());
                Ok(())
            })
            .await?;

            Ok(())
        })
    }

    #[traced_test]
    #[test]
    #[flowtest(_setup: (rt, app, instance_id))]
    fn export_without_folder_unlinked() -> anyhow::Result<()> {
        rt.block_on(async {
            run_export(
                &app,
                instance_id,
                "nofolder_unlinked.zip",
                ExportEntry(HashMap::from([])),
                true,
            )
            .await?;

            check_export(&app, "nofolder_unlinked.zip", |manifest, mut zip| {
                crate::assert_eq_display!(
                    manifest,
                    r#"{
  "formatVersion": 1,
  "game": "minecraft",
  "versionId": "",
  "name": "test",
  "summary": null,
  "files": [],
  "dependencies": {
    "minecraft": "1.16.5",
    "forge": "36.2.34"
  }
}"#
                );

                assert!(zip.by_name("overrides/mods").is_err());
                Ok(())
            })
            .await?;

            Ok(())
        })
    }

    #[traced_test]
    #[test]
    #[flowtest(_setup: (rt, app, instance_id))]
    fn export_with_fake_folder_linked() -> anyhow::Result<()> {
        rt.block_on(async {
            run_export(
                &app,
                instance_id,
                "fakefolder_linked.zip",
                ExportEntry(HashMap::from([(
                    String::from("mods"),
                    Some(ExportEntry(HashMap::from([(
                        String::from("fake-mod.jar"),
                        None,
                    )]))),
                )])),
                true,
            )
            .await?;

            check_export(&app, "fakefolder_linked.zip", |manifest, mut zip| {
                crate::assert_eq_display!(
                    manifest,
                    r#"{
  "formatVersion": 1,
  "game": "minecraft",
  "versionId": "",
  "name": "test",
  "summary": null,
  "files": [],
  "dependencies": {
    "minecraft": "1.16.5",
    "forge": "36.2.34"
  }
}"#
                );

                assert!(zip.by_name("overrides/mods").is_err());
                Ok(())
            })
            .await?;

            Ok(())
        })
    }
}
