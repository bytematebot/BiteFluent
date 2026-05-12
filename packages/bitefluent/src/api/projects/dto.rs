use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub repository_full_name: String,
    pub repository_owner: String,
    pub repository_name: String,
    pub default_branch: String,
    pub source_locale: Option<String>,
    pub locales_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluentFileDto {
    pub path: String,
    pub locale: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectScanDto {
    pub project_id: String,
    pub repository_full_name: String,
    pub files: Vec<FluentFileDto>,
    pub directories: Vec<FluentDirectoryDto>,
    pub suggested_locales_path: Option<String>,
    pub suggested_source_locale: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluentDirectoryDto {
    pub path: String,
    pub files_count: usize,
    pub locales: Vec<String>,
    pub recommended: bool,
    pub score: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWorkspaceDto {
    pub project: ProjectDto,
    pub files: Vec<ProjectWorkspaceFileDto>,
    pub keys: Vec<ProjectWorkspaceKeyDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWorkspaceFileDto {
    pub path: String,
    pub sha: String,
    pub locale: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWorkspaceKeyDto {
    pub key: String,
    pub file_path: String,
    pub locale: Option<String>,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSyncDto {
    pub files_count: usize,
    pub keys_count: usize,
    pub translations_count: usize,
    pub locales_count: usize,
    pub last_synced_at: Option<String>,
    pub sync_status: Option<String>,
    pub sync_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSyncProgressDto {
    pub total_files: usize,
    pub processed_files: usize,
    pub keys_count: usize,
    pub translations_count: usize,
    pub locales_count: usize,
    pub message: String,
    pub done: bool,
    pub error: Option<String>,
}

#[cfg(feature = "server")]
pub fn map_project(project: byteorm_client::Projects) -> ProjectDto {
    ProjectDto {
        id: project.id,
        name: project.name,
        slug: project.slug,
        repository_full_name: project.repository_full_name,
        repository_owner: project.repository_owner,
        repository_name: project.repository_name,
        default_branch: project.default_branch,
        source_locale: project.source_locale,
        locales_path: project.locales_path,
    }
}
