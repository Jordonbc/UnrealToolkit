#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size, LogicalSize};
use tauri::api::dialog;

#[derive(Debug, Clone)]
struct Window {
    window: tauri::Window
}

static mut MAIN_WINDOW: Option<Window> = None;

fn get_main_window() -> tauri::Window {
    unsafe {
        let a = MAIN_WINDOW.clone().unwrap();
        return a.window;
    }
}

fn set_main_window(new_window: tauri::Window) {

    unsafe {MAIN_WINDOW = Some(Window { window: new_window })}
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn open_ue_directory_dialog() -> String {
    dialog::FileDialogBuilder::default()
          .add_filter("Markdown", &["md"])
          .pick_file(|path_buf| match path_buf {
            Some(p) => {}
            _ => {}
          });
          String::from("value")
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 1280.0, height: 720.0 })).expect("Error setting window size!");
            get_main_window().set_min_size(Some(LogicalSize { width: 640.0, height: 360.0 })).expect("Failed to set min size");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_ue_directory_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
