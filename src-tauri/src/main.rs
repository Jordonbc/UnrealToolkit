#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::{fs, env};
use tauri::{Manager, Size, LogicalSize};
use tauri::api::{dialog};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use directories::ProjectDirs;

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
    static ref CONFIG_LOCATION: PathBuf = {
        let dirs = ProjectDirs::from("dev", "", "unrealtoolkit");
        match dirs {
        Some(_) => return dirs.unwrap().config_dir().to_path_buf().join("config.json"),
        None => todo!(),
        }
    };
}

fn get_config() -> ConfigTemplate {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    unsafe {
        return CONFIG.clone().unwrap();
    }
}

fn set_config(new_setting: ConfigTemplate) {
    unsafe {
        let _guard = CONFIG_MUTEX.lock().unwrap();
        CONFIG = Some(new_setting);
    }
    save_config_file();
}

fn reset_config() {
    set_config(ConfigTemplate { ue_directory: String::new() });
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error Creating file!");
}

fn try_create_config_folders() {
    if !CONFIG_LOCATION.exists() {
        let directory_tree = CONFIG_LOCATION.as_path().clone().parent().unwrap();

        println!("Creating folder tree: {}", directory_tree.to_str().unwrap());
        fs::create_dir_all(directory_tree).expect("failed to create folders");
    }
}

fn read_config_file() {
    println!("Reading config file: {}", CONFIG_LOCATION.to_str().unwrap());

    try_create_config_folders();

    let config_result = fs::read_to_string(CONFIG_LOCATION.to_str().unwrap());
    match config_result {
        Ok(_) => {
            let sanitized_config = serde_json::from_str(config_result.unwrap().as_str());
            match sanitized_config {
                Ok(_) => {
                    set_config(sanitized_config.unwrap());
                },
                Err(_) => reset_config(),
            }
        },
        Err(_) => reset_config(),
    }
}

fn save_config_file() {
    println!("Saving config file: {}", CONFIG_LOCATION.to_str().unwrap());
    try_create_config_folders();
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error saving file!");
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
        read_config_file();
        //let config = ConfigTemplate { ue_directory: String::new()};
        //set_config(config);
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
