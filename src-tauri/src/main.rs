#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{env};
use std::path::Path;
use tauri::{Manager, Size, LogicalSize};

use crate::globals::*;
mod globals;
use crate::tauri_commands::*;
mod tauri_commands;
use crate::config::*;
mod config;

fn run_uat(config: Configuration) {
    println!("Running: {}", Path::new(&get_config().ue_directory).join("Engine/Build/BatchFiles/RunUAT.bat").to_str().unwrap());

    //let mut logs: Vec<String> = Vec::new();
    
    for item in config.configuration {
        RUNNING_JOBS.lock().unwrap().push(std::thread::spawn(move ||
        {
            *CLIENT_PACKAGING_STATUS.lock().unwrap() = JobStatus::Running;
            update_frontend();

            let mut foo = Command::new(Path::new(&get_config().ue_directory).join("Engine/Build/BatchFiles/RunUAT.bat"))
            .args(["BuildCookRun",
            &format!("-project={}", &get_project_directory()),
            &format!("-targetplatform={}", item),
            "-pak",
            "-unattended",
            "-prereqs",
            "-cook",
            "-stage",
            "-build",
            "-package",
            &format!("-configuration={}", config.build),
            "-archive"
            ])
            .status().unwrap();

            *CLIENT_PACKAGING_STATUS.lock().unwrap() = JobStatus::Stopped;
            update_frontend();
        }));
    }
}

fn main() {
    read_config_file();
    
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 640.0, height: 660.0 })).expect("Error setting window size!");
            get_main_window().set_min_size(Some(LogicalSize { width: 640.0, height: 660.0 })).expect("Failed to set min size");

            println!("ue directory: {}", get_config().ue_directory);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_ue_directory_dialog, 
            set_ue_directory, 
            get_ue_directory, 
            open_project_directory_dialog,
            get_project_directory,
            set_project_directory,
            set_compiled_output_directory,
            get_compiled_output_directory,
            get_is_source_directory,
            open_output_directory_dialog,
            set_client_configuration,
            get_client_configuration,
            set_server_configuration,
            get_server_configuration,
            package_client,
            package_server,
            get_client_packaging_status,
            get_server_packaging_status
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
