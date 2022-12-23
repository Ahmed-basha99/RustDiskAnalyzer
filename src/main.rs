pub mod helpers;
pub mod structs;
pub mod widgets;

use eframe::egui;
use egui::*;

use crate::helpers::constants::*;
use crate::helpers::app::FileAnalyzerApp;

fn main() {
    // startup options
    let options = eframe::NativeOptions {
        // initial_window_size: Some(Vec2::new(INIT_WIN_WIDTH, INIT_WIN_HEIGHT)),
        // initial_window_pos: Some(Pos2::new(400.0, 400.0)),
        ..Default::default()
    };
    // main app
    eframe::run_native(
        "File Analyzer",
        options,
        Box::new(|_cc| {
            Box::new(FileAnalyzerApp::init())
        }),
    );
}