use egui::*;
use crate::helpers::context::AppContext;
use crate::widgets::collapsing::folder_collapsing;


pub fn left_panel(app_ctx: &mut AppContext,ui: &mut Ui) {
    ui.heading("Directories");
    ScrollArea::both().id_source("left_panel_scroll_area").max_width(300.0).show(ui, |ui| {
        if let Ok(root_dir) = std::fs::read_dir(&app_ctx.selected_root) {
            ui.horizontal(|ui| { ui.add_space(200.0) });

            for dir_result in root_dir {
                if let Ok(dir) = dir_result {
                    let dir_path = dir.path();
                    if dir_path.is_dir() {
                        folder_collapsing(ui, dir_path, &mut app_ctx.selected_dir);
                    }
                }
            }
        } else {
            WidgetText::from("No root access").heading().color(Color32::from_rgba_unmultiplied(255, 0, 0, 255));
        }
        ui.add_space(ui.available_height());
    });
}