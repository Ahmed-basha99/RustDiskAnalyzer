use eframe::egui;
use egui::*;

use crate::helpers::context::AppContext;
use crate::widgets::left_panel::left_panel;
use crate::widgets::right_panel::right_panel;
use crate::widgets::top_panel::top_panel;
// use crate::widgets::chart::chart;

pub struct FileAnalyzerApp {
    context: AppContext,  // atrriubtes
}

// egui update
impl eframe::App for FileAnalyzerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| self.content(ui, ctx));
    }
}

impl FileAnalyzerApp {
    fn new() -> Self {
        Self {
            context: AppContext::new()
        }
    }

    pub fn init() -> Self {
        let mut app = Self::new();
        app.context.init_disks();
        app
    }

    // app content inside window
    fn content(&mut self, ui: &mut Ui, ctx: &Context) { // where is it used
        let app_ctx = &mut self.context;
        ui.vertical(|ui| {
            ui.add_space(8.0);
            // top_panel(app_ctx, ui);
            ui.add_space(12.0);
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    left_panel(app_ctx, ui);
                });
                ui.separator();

                ui.vertical(|ui| {
                    // chart();
                     right_panel(app_ctx, ui, ctx);
                });

            });
        });
    }
}