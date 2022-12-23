use egui::*;
use egui_extras::*;
use crate::helpers::constants::{DISK_INFO_HEADER_RELATIVE_SIZE, DISK_INFO_HEADERS};
use crate::helpers::context::AppContext;

pub fn top_panel(app_ctx: &mut AppContext, ui: &mut Ui) {
    ui.heading("Disks");
    ScrollArea::both().max_height(300.0).show(ui, |ui| {
        let rel_size = ui.available_width() * DISK_INFO_HEADER_RELATIVE_SIZE;
        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Size::exact(20.0))
            .column(Size::initial(120.0).at_least(100.0))
            .column(Size::initial(rel_size))// why repearting
            .column(Size::initial(rel_size))

            .column(Size::initial(rel_size))
            .column(Size::exact(100.0))
            .column(Size::initial(rel_size))
            .resizable(true);
        table.header(24.0, |mut header| {
            header.col(|ui| {
                let _ = ui.radio(true, "");
            });
            for info_header in DISK_INFO_HEADERS {
                header.col(|ui| {
                    ui.label(WidgetText::from(info_header).strong());
                });
            }
        }).body(|mut body| {
            for disk in &mut app_ctx.disks {
                body.row(24.0, |mut row| {
                    row.col(|ui| {
                        let _ = ui.radio(disk.mounted_on == app_ctx.selected_root, "");
                    });
                    row.col(|ui| {
                        ui.label(&disk.file_system);
                    });
                    row.col(|ui| {
                        ui.label(&disk.one_k_blocks);
                    });
                    row.col(|ui| {
                        ui.label(&disk.used);
                    });
                    row.col(|ui| {
                        ui.label(&disk.available);
                    });
                    row.col(|ui| {
                        let progress = (disk.use_percentage.parse::<i32>().unwrap() as f32) / 100.0;
                        ui.add(ProgressBar::new(progress).show_percentage());
                    });
                    row.col(|ui| {
                        ui.label(&disk.mounted_on);
                    });
                });
            }
        });
    });
}