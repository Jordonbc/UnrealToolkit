use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

pub static mut MAIN_WINDOW: Option<Window> = None;
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
    unsafe {
        let a = MAIN_WINDOW.clone().unwrap();
        return a.window;
    }
}

pub fn set_main_window(new_window: tauri::Window) {

    unsafe {MAIN_WINDOW = Some(Window { window: new_window })}
}