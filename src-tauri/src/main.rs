#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::fs;

use tauri::{Manager, Size, LogicalSize};
use tauri::api::dialog;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Window {
    window: tauri::Window
}

static mut MAIN_WINDOW: Option<Window> = None;
static mut CONFIG: Option<ConfigTemplate> = None;

#[derive(Serialize, Deserialize, Clone)]
struct ConfigTemplate {
    ue_directory: String,
}

lazy_static! {
    static ref CONFIG_MUTEX: Mutex<()> = Mutex::new(());
}

fn get_config() -> ConfigTemplate {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    unsafe {
        return CONFIG.clone().unwrap();
    }
}

fn set_config(new_setting: ConfigTemplate) {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    unsafe {
        CONFIG = Some(new_setting);
    }
}

fn read_config_file() {
    let config_result = fs::read_to_string("config.json");
    match config_result {
        Ok(_) => {
            unsafe {
                CONFIG = serde_json::from_str(config_result.unwrap().as_str()).expect("Error Sanitizing config file!");
            }
        },
        Err(_) => (),
    }

}

impl Default for ConfigTemplate {
    fn default() -> ConfigTemplate {
        let config_result = fs::read_to_string("config.json");
        match config_result {
            Ok(_) => {
                let config_as_string: ConfigTemplate = serde_json::from_str(config_result.unwrap().as_str()).expect("Error Sanitizing config file!");
            },
            Err(_) => {
                fs::File::create("config.json");

            },
        }


        ConfigTemplate {
            ue_directory: String::new(),
        }
    }
}

#[tauri::command]
fn get_ue_directory() -> String {
    get_config().ue_directory;
    print!("Returning: {}", get_config().ue_directory);

    return get_config().ue_directory;
}

#[tauri::command]
fn set_ue_directory(new_directory: String) {
    println!("Setting UE_DIRECTORY_LOCATION: {}", new_directory);
    let mut config = get_config();
    config.ue_directory = new_directory;
    set_config(config);
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
    {
        let config = ConfigTemplate { ue_directory: String::new()};
        set_config(config);
    }
    
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 1280.0, height: 720.0 })).expect("Error setting window size!");
            get_main_window().set_min_size(Some(LogicalSize { width: 640.0, height: 360.0 })).expect("Failed to set min size");

            println!("ue directory: {}", get_config().ue_directory);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_ue_directory_dialog, set_ue_directory, get_ue_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
