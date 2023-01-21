use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fmt;
use std::sync::Mutex;
use std::path::PathBuf;
use std::thread::JoinHandle;
use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UEBuild {
    Shipping,
    Test,
    Development,
}

impl fmt::Display for UEBuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub configuration: Vec<String>,
    pub build: UEBuild,
    pub remove_crash_reporter: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JobStatus {
    Running,
    Stopped,
}

pub static MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
pub static CONFIG: Mutex<Option<ConfigTemplate>> = Mutex::new(None);
pub static PROJECT_DIRECTORY: Mutex<Option<String>> = Mutex::new(None);
pub static COMPILED_OUTPUT_DIRECTORY: Mutex<Option<String>> = Mutex::new(None);

pub static CLIENT_PACKAGING_STATUS: Mutex<JobStatus> = Mutex::new(JobStatus::Stopped);
pub static SERVER_PACKAGING_STATUS: Mutex<JobStatus> = Mutex::new(JobStatus::Stopped);

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigTemplate {
    pub ue_directory: String,
    pub ue_source: bool
}

lazy_static! {
    pub static ref CONFIG_LOCATION: PathBuf = {
        let dirs = ProjectDirs::from("dev", "", "unrealtoolkit");
        match dirs {
        Some(_) => return dirs.unwrap().config_dir().to_path_buf().join("config.json"),
        None => todo!(),
        }
    };

    pub static ref RUNNING_JOBS: Mutex<Vec<JoinHandle<()>>> = Mutex::new(Vec::new());

    pub static ref CLIENT_CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration { configuration: ["win64".to_string()].to_vec(),
    build: UEBuild::Shipping,
    remove_crash_reporter: true
});

pub static ref SERVER_CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration { configuration: ["win64".to_string()].to_vec(),
    build: UEBuild::Test,
    remove_crash_reporter: false
});
}

pub fn get_main_window() -> tauri::Window {
    MAIN_WINDOW.lock().unwrap().clone().expect("Error: MAIN_WINDOW is not set!").window
}

pub fn set_main_window(new_window: tauri::Window) {
    *MAIN_WINDOW.lock().unwrap() = Some(Window { window: new_window });
}

pub fn update_frontend() {
    println!("Updating frontend!");
    get_main_window().emit_all("update_frontend", true).expect("Error Sending directory changed event to frontend!");
}