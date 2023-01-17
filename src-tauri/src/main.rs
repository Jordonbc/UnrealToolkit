#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size, LogicalSize};

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
fn greet(name: &str) -> String {
    get_main_window().set_size(Size::Logical(LogicalSize { width: 640.0, height: 360.0 })).expect("error, ");
    format!("Hello, {}! You've been greeted from Rust!", get_main_window().label())
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 1280.0, height: 720.0 })).expect("error, ");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
