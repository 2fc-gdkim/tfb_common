use super::sm_file_info::SMFileInfo;

#[derive(serde::Serialize, Debug, Clone)]
pub struct SMFolderInfo {
    pub name: String,
    pub path: String,
    pub file_list: Vec<SMFileInfo>,
    pub folder_list: Vec<SMFolderInfo>
}

impl SMFolderInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            file_list: Vec::new(),
            folder_list: Vec::new(),
        }
    }

    pub fn new_with_data(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            file_list: Vec::new(),
            folder_list: Vec::new(),
        }
    }

}