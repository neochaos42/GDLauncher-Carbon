use crate::serialize_as_raw_json;
use carbon_macro::into_query_parameters;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{ClassId, ModLoaderType};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ModSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModSearchSortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSearchParameters {
    pub query: ModSearchParametersQuery,
}

#[into_query_parameters]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSearchParametersQuery {
    pub game_id: i32,
    pub search_filter: Option<String>,
    pub game_version: Option<String>,
    #[serde(serialize_with = "serialize_as_raw_json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<Vec<i32>>,
    pub sort_order: Option<ModSearchSortOrder>,
    pub sort_field: Option<ModSearchSortField>,
    pub class_id: Option<ClassId>,
    #[serde(serialize_with = "serialize_as_raw_json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader_types: Option<Vec<ModLoaderType>>,
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFilesParameters {
    pub mod_id: i32,
    pub query: ModFilesParametersQuery,
}

#[into_query_parameters]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFilesParametersQuery {
    pub game_version: Option<String>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFileParameters {
    pub mod_id: i32,
    pub file_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModDescriptionParameters {
    pub mod_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModParameters {
    pub mod_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModsParameters {
    pub body: ModsParametersBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModsParametersBody {
    pub mod_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesParameters {
    pub body: FilesParametersBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesParametersBody {
    pub file_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFileChangelogParameters {
    pub mod_id: i32,
    pub file_id: i32,
}
