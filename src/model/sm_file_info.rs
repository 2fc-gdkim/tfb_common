use std::fs::Metadata;
use std::path::PathBuf;

#[derive(serde::Serialize, Debug, Clone)]
pub struct SMFileInfo {
    pub name: String,
    pub size: u64,
}

impl SMFileInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            size: 0u64,
        }
    }

    pub fn new_with_data(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }

    pub fn new_with_file(metadata: &Metadata, file_path: &PathBuf) -> Self {
        Self{
            name: file_path.file_name().unwrap().to_string_lossy().to_string(),
            size: metadata.len(),
        }
    }    
}