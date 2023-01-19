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
static mut UE_DIRECTORY_LOCATION: String = String::new();

fn get_ue_directory() -> String {
    unsafe {
        return UE_DIRECTORY_LOCATION.clone();
    }
}

#[tauri::command]
fn set_ue_directory(new_directory: String) {

    println!("Setting UE_DIRECTORY_LOCATION: {}", new_directory);
    unsafe {UE_DIRECTORY_LOCATION = new_directory}
}

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
async fn open_ue_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .pick_folder(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    set_ue_directory(path_buf.to_str().unwrap().to_string());
                    println!("Emitting to fromtend: {}", get_ue_directory());
                    get_main_window().emit_all("ue_directory_changed", get_ue_directory()).expect("Error Sending directory changed event to frontend!");
                },
                None => {}
            }
        );
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 1280.0, height: 720.0 })).expect("Error setting window size!");
            get_main_window().set_min_size(Some(LogicalSize { width: 640.0, height: 360.0 })).expect("Failed to set min size");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_ue_directory_dialog, set_ue_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
