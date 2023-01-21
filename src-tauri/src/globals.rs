use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::sync::Mutex;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

pub static MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
//pub static mut MAIN_WINDOW: Option<Window> = None;
pub static mut CONFIG: Option<ConfigTemplate> = None;
pub static mut PROJECT_DIRECTORY: Option<String> = None;
pub static mut COMPILED_OUTPUT_DIRECTORY: Option<String> = None;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigTemplate {
    pub ue_directory: String,
    pub ue_source: bool
}

lazy_static! {
    pub static ref CONFIG_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref CONFIG_LOCATION: PathBuf = {
        let dirs = ProjectDirs::from("dev", "", "unrealtoolkit");
        match dirs {
        Some(_) => return dirs.unwrap().config_dir().to_path_buf().join("config.json"),
        None => todo!(),
        }
    };
}

pub fn get_main_window() -> tauri::Window {
    MAIN_WINDOW.lock().unwrap().clone().unwrap().window
}

pub fn set_main_window(new_window: tauri::Window) {
    *MAIN_WINDOW.lock().unwrap() = Some(Window { window: new_window });
}

pub fn update_frontend() {
    println!("Updating frontend!");
    get_main_window().emit_all("update_frontend", true).expect("Error Sending directory changed event to frontend!");
}