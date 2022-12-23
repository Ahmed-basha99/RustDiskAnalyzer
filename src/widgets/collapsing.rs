use std::ffi::OsStr;
use std::path::PathBuf;
use egui::*;
use crate::helpers::constants::ERROR_STR;

pub fn folder_collapsing(ui: &mut Ui, path: PathBuf, selected_path: &mut String) {
    let file_name = path.file_name()
        .unwrap_or(OsStr::new(ERROR_STR)).to_str()
        .unwrap_or(ERROR_STR);

    let path_str = path.to_str().unwrap_or(ERROR_STR);
    let mut heading = WidgetText::from(file_name);
    if path_str == selected_path {
        heading = heading.color(Color32::from_rgb(0, 255, 255));
    }
    if ui.collapsing(heading, |ui| {
        if let Ok(read_dir) = std::fs::read_dir(&path) {
            for dir_entry_result in read_dir {
                if let Ok(dir_entry) = dir_entry_result {
                    folder_collapsing(ui, dir_entry.path(), selected_path);
                }
            }
        }
    }).header_response.clicked() {
        *selected_path = path_str.to_string();
    }
}
