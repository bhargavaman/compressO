use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionResult {
    pub file_name: String,
    pub file_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub path: String,
    pub file_name: String,
    pub mime_type: String,
    pub extension: String,
    pub size: u64,
}
