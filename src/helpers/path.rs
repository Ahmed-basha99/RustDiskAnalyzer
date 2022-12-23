use std::ffi::OsStr;
use std::path::PathBuf;
use crate::helpers::constants::ERROR_STR;


pub fn file_name(path: &PathBuf) -> String {
    path.file_name().unwrap_or(OsStr::new(ERROR_STR)).to_str().unwrap().to_string()
}

pub fn path_str(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

pub fn file_type(path: &PathBuf) -> String {
    path.extension().unwrap_or(OsStr::new("<unknown>")).to_str().unwrap().to_string()
}

pub fn file_size(path: &PathBuf) -> u64 {
    return match std::fs::metadata(path) {
        Ok(md) => { md.len() }
        Err(_) => { 0 }
    };
}