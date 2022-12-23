use std::process::Command;
use crate::helpers::sort::{SortArrangement, SortFile};
use crate::structs::disk::Disk;

// app data and settings
pub struct AppContext {
    pub selected_root: String,
    pub selected_dir: String,
    pub selected_file: String,
    pub sort_file: SortFile,
    pub sort_arrangement: SortArrangement,
    pub show_include_sub_folders_window: bool,
    pub include_sub_folders: bool,
    pub files_count: u64,
    pub search: String,
    pub disks: Vec<Disk>,
}

impl AppContext {
    pub(crate) fn new() -> AppContext {
        AppContext {
            selected_root: String::from("/"),
            selected_dir: String::from("/"),
            selected_file: "".to_string(),
            sort_file: SortFile::Name,
            sort_arrangement: SortArrangement::Ascending,
            show_include_sub_folders_window: false,
            include_sub_folders: false,
            files_count: 0,
            search: "".to_string(),
            disks: Vec::new(),
        }
    }

    // initialize disks
    pub fn init_disks(&mut self) {
        let mut disks: Vec<Disk> = Vec::new();
        let process = Command::new("df")
            .output()
            .ok()
            .expect("Failed to execute");
        let out = String::from_utf8(process.stdout)
            .ok()
            .expect("Failed to read");
        for line in out.split("\n") {
            if line.starts_with("/") {
                let disk = Disk::from_df_line(line);
                disks.push(disk);
            }
        }
        self.disks = disks;
    }
}
