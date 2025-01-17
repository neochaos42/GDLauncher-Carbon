use carbon_rt_path::InstancePath;
use daedalus::minecraft::VersionManifest;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, path::PathBuf};
use tracing::info;

use crate::{modrinth::UtcDateTime, ModChannel};

pub mod filters;
pub mod manifest;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedModsResponse {
    pub featured: Vec<Mod>,
    pub popular: Vec<Mod>,
    pub recently_updated: Vec<Mod>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: i32,
    pub game_id: i32,
    pub mod_id: i32,
    pub is_available: bool,
    pub display_name: String,
    pub file_name: String,
    pub release_type: FileReleaseType,
    pub file_status: FileStatus,
    pub hashes: Vec<FileHash>,
    pub file_date: UtcDateTime,
    pub file_length: u32,
    pub download_count: u32,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<i32>,
    pub alternate_file_id: Option<i32>,
    pub is_server_pack: Option<bool>,
    pub server_pack_file_id: Option<i32>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<String>, // Consider using a datetime library for date-time representation
    pub file_fingerprint: u32,
    pub modules: Vec<FileModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: i32,
    pub relation_type: FileRelationType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FileReleaseType {
    Stable = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileModule {
    pub name: String,
    pub fingerprint: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFuzzyMatch {
    pub id: i32,
    pub file: File,
    pub latest_files: Vec<File>,
    pub fingerprints: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFuzzyMatchResult {
    pub fuzzy_matches: Vec<FingerprintFuzzyMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintMatch {
    pub id: i32,
    pub file: File,
    pub latest_files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintsMatchesResult {
    pub is_cache_built: bool,
    pub exact_matches: Vec<FingerprintMatch>,
    pub exact_fingerprints: Vec<u32>,
    pub partial_matches: Vec<FingerprintMatch>,
    pub partial_match_fingerprints: HashMap<String, Vec<u64>>,
    pub installed_fingerprints: Vec<u32>,
    pub unmatched_fingerprints: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderFingerprint {
    pub foldername: String,
    pub fingerprints: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub date_modified: String, // date-time
    pub assets: GameAssets,
    pub status: CoreStatus,
    pub api_status: CoreApiStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftGameVersion {
    pub id: i32,
    pub game_version_id: i32,
    pub version_string: String,
    pub jar_download_url: String,
    pub json_download_url: String,
    pub approved: bool,
    pub date_modified: String, // date-time
    pub game_version_type_id: i32,
    pub game_version_status: GameVersionStatus,
    pub game_version_type_status: GameVersionTypeStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftModLoaderIndex {
    pub name: String,
    pub game_version: String,
    pub latest: bool,
    pub recommended: bool,
    pub date_modified: String, // date-time
    pub mod_loader_type: ModLoaderType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftModLoaderVersion {
    pub id: i32,
    pub game_version_id: i32,
    pub minecraft_game_version_id: i32,
    pub forge_version: String,
    pub name: String,
    pub mod_loader_type: ModLoaderType,
    pub download_url: String,
    pub filename: String,
    pub install_method: ModLoaderInstallMethod,
    pub latest: bool,
    pub recommended: bool,
    pub approved: bool,
    pub date_modified: String, // date-time
    pub maven_version_string: String,
    pub version_json: String,
    pub libraries_install_location: String,
    pub minecraft_version: String,
    pub additional_files_json: String,
    pub mod_loader_game_version_id: i32,
    pub mod_loader_game_version_type_id: i32,
    pub mod_loader_game_version_status: GameVersionStatus,
    pub mod_loader_game_version_type_status: GameVersionTypeStatus,
    pub mc_game_version_id: i32,
    pub mc_game_version_type_id: i32,
    pub mc_game_version_status: GameVersionStatus,
    pub mc_game_version_type_status: GameVersionTypeStatus,
    pub install_profile_json: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub links: ModLinks,
    pub summary: String,
    pub status: ModStatus,
    pub download_count: u32,
    pub is_featured: bool,
    pub primary_category_id: i32,
    pub categories: Vec<Category>,
    pub class_id: Option<ClassId>,
    pub authors: Vec<ModAuthor>,
    pub logo: Option<ModAsset>,
    pub screenshots: Vec<ModAsset>,
    pub main_file_id: i32,
    pub latest_files: Vec<File>,
    pub latest_files_indexes: Vec<FileIndex>,
    pub date_created: String,  // date-time
    pub date_modified: String, // date-time
    pub date_released: String, // date-time
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: i32,
    pub is_available: bool,
    pub thumbs_up_count: i32,
}

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum ClassId {
    Mods = 6,
    ResourcePacks = 12,
    Modpacks = 4471,
    Customizations = 4546,
    BukkitPlugins = 5,
    Worlds = 17,
    Addons = 4559,
    Shaders = 6552,
    Other(u16),
}

impl ClassId {
    pub fn into_path(
        self,
        instance_path: &InstancePath,
        game_version: String,
        mc_manifest: &VersionManifest,
    ) -> PathBuf {
        match self {
            Self::ResourcePacks => {
                // anything older than 1.6.1 is in the texturepacks folder
                let index_1_6_1 = mc_manifest
                    .versions
                    .iter()
                    .position(|v| v.id == "1.6.1")
                    .unwrap_or(0);

                let manifest_version = mc_manifest
                    .versions
                    .iter()
                    .position(|v| v.id == game_version)
                    .unwrap_or(0);

                info!(
                    "Index 1.6.1: {} | manifest index current game version ({}): {}",
                    index_1_6_1, game_version, manifest_version
                );

                // current game version needs to be below 1.6.1, so the index needs to be higher
                if manifest_version > index_1_6_1 {
                    instance_path.get_texturepacks_path()
                } else {
                    instance_path.get_resourcepacks_path()
                }
            }
            Self::BukkitPlugins => instance_path.get_plugins_path(),
            Self::Worlds => instance_path.get_saves_path(),
            Self::Shaders => instance_path.get_shaderpacks_path(),
            _ => instance_path.get_mods_path(),
        }
    }
}

impl<'de> Deserialize<'de> for ClassId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u16::deserialize(deserializer)?;
        match n {
            6 => Ok(Self::Mods),
            12 => Ok(Self::ResourcePacks),
            4471 => Ok(Self::Modpacks),
            4546 => Ok(Self::Customizations),
            5 => Ok(Self::BukkitPlugins),
            17 => Ok(Self::Worlds),
            4559 => Ok(Self::Addons),
            6552 => Ok(Self::Shaders),
            other => Ok(Self::Other(other)),
        }
    }
}

impl Serialize for ClassId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(match self {
            Self::Mods => 6,
            Self::ResourcePacks => 12,
            Self::Modpacks => 4471,
            Self::Customizations => 4546,
            Self::BukkitPlugins => 5,
            Self::Worlds => 17,
            Self::Addons => 4559,
            Self::Shaders => 6552,
            Self::Other(other) => *other,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModDependencies {
    pub id: i32,
    pub mod_id: i32,
    pub file_id: i32,
    pub file_dependency_id: i32,
    pub type_id: i32,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFileModule {
    pub folder_name: String,
    pub fingerprint: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFileStat {
    pub mod_id: i32,
    pub file_id: i32,
    pub timestamp: String, // date-time
    pub total_downloads: u32,
    pub downloads: u32,
    pub update_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFileVersion {
    pub id: i32,
    pub mod_id: i32,
    pub file_id: i32,
    pub game_version_id: i32,
    pub game_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version_name: String,
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String, // date-time
    pub game_version_type_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameAssets {
    pub game: String,
    pub logo: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameVersionStatus {
    Approved = 1,
    Deleted = 2,
    New = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameVersionTypeStatus {
    Normal = 1,
    Deleted = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum ModLoaderType {
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
    Other(u8),
}

impl<'de> Deserialize<'de> for ModLoaderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u8::deserialize(deserializer)?;
        match n {
            1 => Ok(Self::Forge),
            2 => Ok(Self::Cauldron),
            3 => Ok(Self::LiteLoader),
            4 => Ok(Self::Fabric),
            5 => Ok(Self::Quilt),
            6 => Ok(Self::NeoForge),
            other => Ok(Self::Other(other)),
        }
    }
}

impl Serialize for ModLoaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            Self::Forge => 1,
            Self::Cauldron => 2,
            Self::LiteLoader => 3,
            Self::Fabric => 4,
            Self::Quilt => 5,
            Self::NeoForge => 6,
            Self::Other(other) => *other,
        })
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderInstallMethod {
    ForgeInstaller = 1,
    ForgeJarInstall = 2,
    ForgeInstallerV2 = 3,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    pub website_url: Option<String>,
    pub wiki_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub icon_url: Option<String>,
    pub date_modified: String,
    pub is_class: Option<bool>,
    pub class_id: Option<i32>,
    pub parent_category_id: Option<i32>,
    pub display_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    pub id: i32,
    pub mod_id: i32,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: i32,
    pub filename: String,
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<i32>,
    pub mod_loader: Option<ModLoaderType>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DependencyType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurseForgeResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

impl From<FileReleaseType> for ModChannel {
    fn from(value: FileReleaseType) -> Self {
        match value {
            FileReleaseType::Alpha => ModChannel::Alpha,
            FileReleaseType::Beta => ModChannel::Beta,
            FileReleaseType::Stable => ModChannel::Stable,
        }
    }
}

impl From<ModChannel> for FileReleaseType {
    fn from(value: ModChannel) -> Self {
        match value {
            ModChannel::Alpha => FileReleaseType::Alpha,
            ModChannel::Beta => FileReleaseType::Beta,
            ModChannel::Stable => FileReleaseType::Stable,
        }
    }
}
