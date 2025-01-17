use serde::{Deserialize, Serialize};
use specta::Type;

use carbon_platforms::curseforge::{
    Category, CurseForgeResponse, File, MinecraftModLoaderIndex, Mod,
};

use super::structs::{
    CFFECategory, CFFEFile, CFFEMinecraftModLoaderIndex, CFFEMod, CFFEPagination,
};

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModSearchResponse {
    pub data: Vec<CFFEMod>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<Mod>>> for FEModSearchResponse {
    fn from(response: CurseForgeResponse<Vec<Mod>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModloadersResponse {
    pub data: Vec<CFFEMinecraftModLoaderIndex>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<MinecraftModLoaderIndex>>> for FEModloadersResponse {
    fn from(response: CurseForgeResponse<Vec<MinecraftModLoaderIndex>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FECategoriesResponse {
    pub data: Vec<CFFECategory>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<Category>>> for FECategoriesResponse {
    fn from(response: CurseForgeResponse<Vec<Category>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModResponse {
    pub data: CFFEMod,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Mod>> for FEModResponse {
    fn from(response: CurseForgeResponse<Mod>) -> Self {
        Self {
            data: response.data.into(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModsResponse {
    pub data: Vec<CFFEMod>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<Mod>>> for FEModsResponse {
    fn from(response: CurseForgeResponse<Vec<Mod>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModDescriptionResponse {
    pub data: String,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<String>> for FEModDescriptionResponse {
    fn from(response: CurseForgeResponse<String>) -> Self {
        Self {
            data: response.data,
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModFileResponse {
    pub data: CFFEFile,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<File>> for FEModFileResponse {
    fn from(response: CurseForgeResponse<File>) -> Self {
        Self {
            data: response.data.into(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModFilesResponse {
    pub data: Vec<CFFEFile>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<File>>> for FEModFilesResponse {
    fn from(response: CurseForgeResponse<Vec<File>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEFilesResponse {
    pub data: Vec<CFFEFile>,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<Vec<File>>> for FEFilesResponse {
    fn from(response: CurseForgeResponse<Vec<File>>) -> Self {
        Self {
            data: response.data.into_iter().map(Into::into).collect(),
            pagination: response.pagination.map(Into::into),
        }
    }
}

#[derive(Type, Debug, Deserialize, Serialize)]
pub struct FEModFileChangelogResponse {
    pub data: String,
    pub pagination: Option<CFFEPagination>,
}

impl From<CurseForgeResponse<String>> for FEModFileChangelogResponse {
    fn from(response: CurseForgeResponse<String>) -> Self {
        Self {
            data: response.data,
            pagination: response.pagination.map(Into::into),
        }
    }
}
