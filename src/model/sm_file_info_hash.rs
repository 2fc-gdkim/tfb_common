use std::fs::Metadata;
use std::path::PathBuf;
use crate::utils::{hash, date};

#[derive(serde::Serialize, Debug, Clone)]
pub struct SMFileInfoHash {
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub create_time: String,
    pub last_write_time: String
}

impl SMFileInfoHash {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            size: 0u64,
            hash: String::new(),
            create_time: String::new(),
            last_write_time: String::new(),
        }
    }

    pub fn new_with_data(name: &str, size: u64, hash: &str, create_time: &str, last_write_time: &str) -> Self {
        Self {
            name: name.to_string(),
            size: size,
            hash: hash.to_string(),
            create_time: create_time.to_string(),
            last_write_time: last_write_time.to_string(),
        }
    }

    pub fn new_with_file(metadata: &Metadata, file_path: &PathBuf, get_hash: bool) -> Self {
        let mut hash_value=String::new();
        if get_hash {
            hash_value = hash::get_file_hash256(&file_path.to_string_lossy().to_string());
        }

        Self{
            name: file_path.file_name().unwrap().to_string_lossy().to_string(),
            size: metadata.len(),
            hash: hash_value,
            create_time: date::system_time_to_string(metadata.created().unwrap()),
            last_write_time: date::system_time_to_string(metadata.modified().unwrap()),
        }
    }    
}