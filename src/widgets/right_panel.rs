use egui::*;
use egui_extras::*;
use crate::helpers::constants::FILE_INFO_HEADERS;
use std::sync::{Arc, Mutex};

use crate::helpers::context::AppContext;
use crate::helpers::path::{file_name, path_str};
use crate::helpers::sort::{SortArrangement, SortFile};
use std::thread;

use crate::structs::file::File;
use fltk::{prelude::*, *};

pub fn right_panel(app_ctx: &mut AppContext, ui: &mut Ui, ctx: &Context) {
    let mut files = Vec::new();
    // files.push(File{name : String::from("dir1"), size : 40, path : String::from(""), file_type: String::from("")});


    ui.heading("Files");
    ui.horizontal(|ui| {
        ui.label("Sort by");
        ComboBox::new("sort_by_combo_box", "")
            .selected_text(format!("{}", app_ctx.sort_file.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app_ctx.sort_file, SortFile::Name, SortFile::Name.to_string());
                ui.selectable_value(&mut app_ctx.sort_file, SortFile::Size, SortFile::Size.to_string());
                ui.selectable_value(&mut app_ctx.sort_file, SortFile::Type, SortFile::Type.to_string());
            });
        ui.label("Order");
        ComboBox::new("order_combo_box", "")
            .selected_text(format!("{}", app_ctx.sort_arrangement.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app_ctx.sort_arrangement, SortArrangement::Ascending, SortArrangement::Ascending.to_string());
                ui.selectable_value(&mut app_ctx.sort_arrangement, SortArrangement::Descending, SortArrangement::Descending.to_string());
            });

        // if app_ctx.include_sub_folders {
        //     ui.checkbox(&mut app_ctx.include_sub_folders, "Include sub-folders");
        // } else if !app_ctx.show_include_sub_folders_window {
        //     ui.checkbox(&mut app_ctx.show_include_sub_folders_window, "Include sub-folders");
        // } else {
        //     ui.checkbox(&mut false, "Include sub-folders");
        // }

        let window = Window::new("Include sub-folders")
            .id(Id::new("include_sub_folder_window")) // required since we change the title
            .title_bar(true).default_pos(ui.next_widget_position());

        // if app_ctx.show_include_sub_folders_window {
        //     window.show(ctx, |ui|
        //         {
        //             ui.vertical_centered_justified(|ui| {
        //                 ui.label(format!("Do you want to include files in sub-folders in \"{}\"?\n\
        //             This might take a moment", app_ctx.selected_dir));
        //                 if ui.button("Yes").clicked() {
        //                     app_ctx.include_sub_folders = true;
        //                     app_ctx.show_include_sub_folders_window = false;
        //                 }
        //                 if ui.button("No").clicked() {
        //                     app_ctx.show_include_sub_folders_window = false;
        //                 }
        //             });
        //         });
        // }
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("Current path");
        ui.add(TextEdit::singleline(&mut app_ctx.selected_dir).desired_width(ui.available_width()))
    });
    ui.horizontal(|ui| {
        ui.label("Search");
        ui.add(TextEdit::singleline(&mut app_ctx.search).desired_width(ui.available_width()));
    });

    ui.add_space(4.0);
    // if app_ctx.include_sub_folders && app_ctx.files_count > 50 {
    //     ui.label(format!("Showing first 50 results out of {}", app_ctx.files_count));
    // }

    if ui.button("open chart").clicked(){
        find_files(&app_ctx, &app_ctx.selected_dir, &mut files);
        chart(&files);
           // frame.close();
    }
    ui.add_space(4.0);

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(Layout::left_to_right(Align::Center))
        .column(Size::remainder())
        .column(Size::remainder())
        .column(Size::remainder())
        .column(Size::initial(100.0))
        .resizable(true);
    table.header(24.0, |mut header| {
        for header_text in FILE_INFO_HEADERS {
            header.col(|ui| {
                ui.label(WidgetText::from(header_text).strong());
            });
        }
    }).body(|mut body| {

        // files = Vec::new();
        // let mut file = new (vec));

        find_files(&app_ctx, &app_ctx.selected_dir, &mut files);

        // chart(&files);
        // begining of thread
        //
        // chart(&files);
        // let x = files.clone();
        // let handle = thread::spawn(move || chart(&x));

        match app_ctx.sort_file {
            SortFile::Name => { files.sort_by_key(|f| f.name.clone()); }
            SortFile::Type => { files.sort_by_key(|f| f.file_type.clone()); }
            SortFile::Size => { files.sort_by_key(|f| f.size); }
        }
        if app_ctx.sort_arrangement == SortArrangement::Descending {
            files.reverse();
        }
        if files.is_empty() {
            body.row(24.0, |mut row| {
                row.col(|ui| {
                    ui.label("<No file>");
                });
                row.col(|_ui| {});
                row.col(|_ui| {});
                row.col(|_ui| {});
            });
        } else {
            // if app_ctx.include_sub_folders {
            //     app_ctx.files_count = files.len() as u64;
            //     if app_ctx.files_count > 50 {
            //         files.truncate(50);
            //     }
            // }
            for file in &*files {
                body.row(24.0, |mut row| {
                    row.col(|ui| {
                        if ui.selectable_value(&mut app_ctx.selected_file, file.path.to_string(), &file.name).clicked()
                        {}
                    });
                    row.col(|ui| {
                        ui.label(&file.path);
                    });
                    row.col(|ui| {
                        ui.label(&file.file_type);
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::top_down(Align::Max), |ui| {
                            ui.label(&file.size.to_string());
                        });
                    });
                });
            }
        }
    });
}


// recursive function

fn find_files(app_ctx: &&mut AppContext, path: &str, files: &mut Vec<File>) {
    if let Ok(read_dir) = std::fs::read_dir(path) {
        for file in read_dir {
            let path_buf = &file.unwrap().path();
            if path_buf.is_file() {
                if !app_ctx.search.is_empty() {
                    if !file_name(path_buf).contains(&app_ctx.search) {
                        continue;
                    }
                }

            }
            // if app_ctx.include_sub_folders {
            //     if path_buf.is_dir() {
            //         let path_str = &path_str(path_buf);
            //          find_files(app_ctx, path_str, files);
            //     }
            // }
            let file = File::from_path(path_buf);
            files.push(file);
        }
    }
}





//v: &mut Vec<File>

fn chart(files:  &Vec<File>) {
    println!("{:?}", files.len());
    let mut sum : f64 = 0.0;
    let mut i = 0;
    for file in files {
        let size = file.size as f64;
        sum = sum + size;

    }

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Charts");
    let mut chart = misc::Chart::default().size_of_parent();
    chart.set_type(misc::ChartType::Pie);
    chart.set_bounds(0.0, 100.0);
    chart.set_text_size(11);


    let mut iu:u32  = 1000;
    for f in files {
        let mut x = f.size as f64;
        x = x/sum* 100.0;
        chart.add( x, &f.name.to_owned()[..] , enums::Color::from_u32(iu));
        iu = iu+1001001;
        if iu >=400000000  {
            iu = 1000001;
        }
    }


    win.end();
    win.show();


    app.run().unwrap();

    // let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    // let _result = child.wait().unwrap();

}