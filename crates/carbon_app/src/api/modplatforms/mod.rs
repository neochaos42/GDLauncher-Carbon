use rspc::RouterBuilder;
use serde::{Deserialize, Serialize};
use specta::Type;
use strum::IntoEnumIterator;
use tracing::info;

use crate::{
    api::{
        keys::modplatforms::{
            CURSEFORGE_GET_CATEGORIES, CURSEFORGE_GET_FILES, CURSEFORGE_GET_MOD,
            CURSEFORGE_GET_MODLOADERS, CURSEFORGE_GET_MODS, CURSEFORGE_GET_MOD_DESCRIPTION,
            CURSEFORGE_GET_MOD_FILE, CURSEFORGE_GET_MOD_FILES, CURSEFORGE_GET_MOD_FILE_CHANGELOG,
            CURSEFORGE_SEARCH, MODRINTH_GET_CATEGORIES, MODRINTH_GET_LOADERS, MODRINTH_GET_PROJECT,
            MODRINTH_GET_PROJECTS, MODRINTH_GET_PROJECT_TEAM, MODRINTH_GET_PROJECT_VERSIONS,
            MODRINTH_GET_TEAM, MODRINTH_GET_VERSION, MODRINTH_GET_VERSIONS, MODRINTH_SEARCH,
            UNIFIED_SEARCH,
        },
        modplatforms::curseforge::structs::CFFEModLoaderType,
        router::router,
    },
    managers::App,
    mirror_into,
};

use self::{
    curseforge::structs::CFFEFile,
    modrinth::structs::{MRFEVersion, MRFEVersionFile},
};

mod curseforge;
mod filters;
mod modrinth;
mod responses;

#[derive(Type, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FESearchAPI {
    Curseforge,
    Modrinth,
}

#[derive(Type, Debug, Deserialize, Serialize, Clone, Copy)]
#[repr(i32)]
pub enum ModChannel {
    Alpha = 0,
    Beta,
    Stable,
}
impl Default for ModChannel {
    fn default() -> Self {
        Self::Stable
    }
}

impl TryFrom<i32> for ModChannel {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Alpha),
            1 => Ok(Self::Beta),
            2 => Ok(Self::Stable),
            _ => Err(anyhow::anyhow!(
                "Invalid mod channel id {value} not in range 0..=2"
            )),
        }
    }
}

mirror_into!(
    ModChannel,
    carbon_platforms::ModChannel,
    |value| match value {
        Other::Alpha => Self::Alpha,
        Other::Beta => Self::Beta,
        Other::Stable => Self::Stable,
    }
);

#[derive(Type, Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ModPlatform {
    Curseforge,
    Modrinth,
}

mirror_into!(
    ModPlatform,
    carbon_platforms::ModPlatform,
    |value| match value {
        Other::Curseforge => Self::Curseforge,
        Other::Modrinth => Self::Modrinth,
    }
);

#[derive(Type, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ModChannelWithUsage {
    pub channel: ModChannel,
    pub allow_updates: bool,
}

mirror_into!(
    ModChannelWithUsage,
    carbon_platforms::ModChannelWithUsage,
    |value| {
        Self {
            channel: value.channel.into(),
            allow_updates: value.allow_updates,
        }
    }
);

#[derive(Type, Debug, Deserialize, Serialize, Clone)]
pub struct ModSources {
    pub channels: Vec<ModChannelWithUsage>,
    pub platform_blacklist: Vec<ModPlatform>,
}

mirror_into!(ModSources, carbon_platforms::ModSources, |value| Self {
    channels: value.channels.into_iter().map(Into::into).collect(),
    platform_blacklist: value
        .platform_blacklist
        .into_iter()
        .map(Into::into)
        .collect(),
});

#[derive(Type, Debug, Serialize)]
#[serde(tag = "platform")]
pub enum RemoteVersion {
    Curseforge(CFFEFile),
    Modrinth(MRFEVersion),
}

impl From<carbon_platforms::RemoteVersion> for RemoteVersion {
    fn from(value: carbon_platforms::RemoteVersion) -> Self {
        use carbon_platforms::RemoteVersion as Other;

        match value {
            Other::Curseforge(cf) => Self::Curseforge(cf.into()),
            Other::Modrinth(mr) => Self::Modrinth(mr.into()),
        }
    }
}

pub(super) fn mount() -> RouterBuilder<App> {
    router! {
        // Curseforge
        query CURSEFORGE_SEARCH[app, filters: curseforge::filters::CFFEModSearchParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.search(filters.into()).await?;

            Ok(curseforge::responses::FEModSearchResponse::from(response))
        }

        query CURSEFORGE_GET_MODLOADERS[app, _args: ()] {
            Ok(CFFEModLoaderType::iter().collect::<Vec<_>>())
        }

        query CURSEFORGE_GET_CATEGORIES[app, args: ()] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_categories().await?;

            Ok(curseforge::responses::FECategoriesResponse::from(response))
        }

        query CURSEFORGE_GET_MOD[app, mod_parameters: curseforge::filters::CFFEModParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mod(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModResponse::from(response))
        }

        query CURSEFORGE_GET_MODS[app, mod_parameters: curseforge::filters::CFFEModsParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mods(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModsResponse::from(response))
        }

        query CURSEFORGE_GET_MOD_DESCRIPTION[app, mod_parameters: curseforge::filters::CFFEModDescriptionParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mod_description(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModDescriptionResponse::from(response))
        }

        query CURSEFORGE_GET_MOD_FILE[app, mod_parameters: curseforge::filters::CFFEModFileParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mod_file(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModFileResponse::from(response))
        }

        query CURSEFORGE_GET_MOD_FILES[app, mod_parameters: curseforge::filters::CFFEModFilesParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mod_files(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModFilesResponse::from(response))
        }

        query CURSEFORGE_GET_FILES[app, mod_parameters: curseforge::filters::CFFEFilesParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_files(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEFilesResponse::from(response))
        }

        query CURSEFORGE_GET_MOD_FILE_CHANGELOG[app, mod_parameters: curseforge::filters::CFFEModFileChangelogParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.curseforge.get_mod_file_changelog(mod_parameters.into()).await?;

            Ok(curseforge::responses::FEModFileChangelogResponse::from(response))
        }

        // Modrinth
        query MODRINTH_SEARCH[app, search_params: modrinth::filters::MRFEProjectSearchParameters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.search(search_params.into()).await?;

            Ok(modrinth::responses::MRFEProjectSearchResponse::from(response))

        }
        query MODRINTH_GET_LOADERS[app, _args: ()] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_loaders().await?;

            Ok(modrinth::responses::MRFELoadersResponse::from(response))
        }
        query MODRINTH_GET_CATEGORIES[app, args: () ] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_categories().await?;

            Ok(modrinth::responses::MRFECategoriesResponse::from(response))
        }
        query MODRINTH_GET_PROJECT[app, project: modrinth::filters::MRFEProjectID  ] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_project(project.into()).await?;

            Ok(modrinth::structs::MRFEProject::from(response))
        }
        query MODRINTH_GET_PROJECTS[app, projects: modrinth::filters::MRFEProjectIDs] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_projects(projects.into()).await?;

            Ok(modrinth::responses::MRFEProjectsResponse::from(response))
        }
        query MODRINTH_GET_PROJECT_VERSIONS[app, filters: modrinth::filters::MRFEProjectVersionsFilters] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_project_versions(filters.into()).await?;

            Ok(modrinth::responses::MRFEVersionsResponse::from(response))
        }
        query MODRINTH_GET_VERSION[app, version: modrinth::filters::MRFEVersionID] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_version(version.into()).await?;

            Ok(modrinth::structs::MRFEVersion::from(response))
        }
        query MODRINTH_GET_VERSIONS[app, versions: modrinth::filters::MRFEVersionIDs] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_versions(versions.into()).await?;

            Ok(modrinth::responses::MRFEVersionsResponse::from(response))
        }
        query MODRINTH_GET_PROJECT_TEAM[app, project: modrinth::filters::MRFEProjectID] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_project_team(project.into()).await?;

            Ok(modrinth::responses::MRFETeamResponse::from(response))
        }
        query MODRINTH_GET_TEAM[app, team: modrinth::filters::MRFETeamID] {
            let modplatforms = app.modplatforms_manager();
            let response = modplatforms.modrinth.get_team(team.into()).await?;

            Ok(modrinth::responses::MRFETeamResponse::from(response))
        }

        query UNIFIED_SEARCH[app, search_params: filters::FEUnifiedSearchParameters] {
            match search_params.search_api {
                FESearchAPI::Curseforge => {
                    let search_params: curseforge::filters::CFFEModSearchParameters = search_params.try_into()?;
                    let modplatforms = app.modplatforms_manager();
                    let curseforge_response = modplatforms.curseforge.search(search_params.into()).await?;
                    let fe_curseforge_response = curseforge::responses::FEModSearchResponse::from(curseforge_response);
                    Ok(responses::FEUnifiedSearchResponse::from(fe_curseforge_response))
                }
                FESearchAPI::Modrinth => {
                    let search_params:  modrinth::filters::MRFEProjectSearchParameters = search_params.try_into()?;
                    let modplatforms = app.modplatforms_manager();
                    let modrinth_response = modplatforms.modrinth.search(search_params.into()).await?;
                    let fe_modrinth_response = modrinth::responses::MRFEProjectSearchResponse::from(modrinth_response);
                    Ok(responses::FEUnifiedSearchResponse::from(fe_modrinth_response))
                }
            }
        }
    }
}
