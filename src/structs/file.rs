use std::path::PathBuf;
use crate::helpers::path::*;

pub struct File{
    pub name: String,
    pub path: String,
    pub size: u64,
    pub file_type: String,
}


impl File {
    pub(crate) fn from_path(path: &PathBuf) -> File {
        File {
            name: file_name(path),
            path: path_str(path),
            size: file_size(path),
            file_type: file_type(path),
        }
    }
}
