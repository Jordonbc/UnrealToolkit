use log::{trace, error, info};
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;
use tauri::Manager;
use std::fmt;

use std::path::PathBuf;
use std::sync::Mutex;
use std::thread::JoinHandle;
use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum UEBuild {
    #[default] Shipping,
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
    pub is_server: bool,
}

impl Configuration {
    fn new() -> Self {
        Configuration { configuration: [String::from("Win64")].to_vec(), build: UEBuild::default(), remove_crash_reporter: false, is_server: false }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Running,
    Waiting,
    Stopped,
    Failed,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueElement {
    pub job_status: JobStatus,
    pub command: String,
    pub args: Vec<String>,
}

pub static MAIN_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
pub static MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);

pub static CONFIG: Mutex<Option<ConfigTemplate>> = Mutex::new(None);
pub static PROJECT_DIRECTORY: Mutex<Option<String>> = Mutex::new(None);
pub static COMPILED_OUTPUT_DIRECTORY: Mutex<Option<String>> = Mutex::new(None);

pub static mut PACKAGING_STATUS: JobStatus = JobStatus::Stopped;
pub static SERVER_PACKAGING_STATUS: Mutex<JobStatus> = Mutex::new(JobStatus::Stopped);

pub static mut PACKAGING_THREAD: Option<SharedChild> = None;

pub static mut QUEUE: Vec<QueueElement> = vec![];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigTemplate {
    pub ue_directory: String,
    pub ue_source: bool
}

impl fmt::Display for ConfigTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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

    pub static ref CLIENT_CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::new());
    pub static ref SERVER_CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::new());
}

pub fn get_window(window_label: &str) -> Option<tauri::Window> {
    trace!("getting current window reference");

    MAIN_HANDLE.lock().unwrap().clone().unwrap().get_window(window_label)
}

pub fn set_main_handle(app_handle: tauri::AppHandle) {
    *MAIN_HANDLE.lock().unwrap() = Some(app_handle);
}

pub fn get_main_window() -> tauri::Window {
    get_window("main").unwrap()
}

pub fn set_main_window(new_window: tauri::Window) {
    trace!("Setting main Window: {}", new_window.label());
    *MAIN_WINDOW.lock().unwrap() = Some(Window { window: new_window });
}

pub fn update_frontend() {
    info!("Updating frontend!");
    get_main_window().emit_all("update_frontend", true).expect("Error Sending directory changed event to frontend!");
}