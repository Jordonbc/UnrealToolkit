#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use log::debug;
use log::error;
use log::info;
use log::trace;
use log4rs;

use std::path::Path;
use std::process::Command;
use std::env;
use shared_child::SharedChild;
use tauri::{Manager, Size, LogicalSize};

use crate::globals::*;
mod globals;
use crate::tauri_commands::*;
mod tauri_commands;
use crate::config::*;
mod config;

fn kill_process_tree(pid: u32) {
    if cfg!(target_os = "windows") {
        info!("Killing via taskkill...");
        Command::new("taskkill")
            .args(["/pid", &pid.to_string(), "/T", "/F"])
            .output()
            .expect("Failed to kill process tree");
    } else {
        info!("Killing via pkill...");
        Command::new("pkill")
            .arg("-P")
            .arg(pid.to_string())
            .output()
            .expect("Failed to kill process tree");
    }
}

fn stop_queue() {
    info!("Stopping thread");
    std::thread::spawn(||
        {
        let thread = unsafe {&PACKAGING_THREAD};
        match thread.as_ref() {
            Some(_) => {
                info!("Killing: {}", thread.as_ref().unwrap().id());
                kill_process_tree(thread.as_ref().unwrap().id());
                
                unsafe {PACKAGING_THREAD = None};

                unsafe {PACKAGING_STATUS = JobStatus::Failed;}
                update_frontend();
        },
            None => error!("Failed to get thread!"),
        }
    });
}

fn run_queue() {
    stop_queue();
    
    info!("Running: {}", Path::new(&get_config().ue_directory).join("Engine/Build/BatchFiles/RunUAT.bat").to_str().unwrap());
    
    std::thread::spawn(||
    {
        unsafe {PACKAGING_STATUS = JobStatus::Running;}
        update_frontend();

        let mut index = 0;
        while index < unsafe {QUEUE.len()} {
            let queued_item = unsafe {&mut QUEUE[index]};
            if queued_item.job_status == JobStatus::Waiting {
                queued_item.job_status = JobStatus::Running;

                unsafe {PACKAGING_THREAD = Some(SharedChild::spawn(Command::new(&queued_item.command)
                .args(&queued_item.args)).unwrap())};

                info!("Locking thread, waiting for UAT to complete packaging");
                unsafe {PACKAGING_THREAD.as_ref().unwrap().wait().unwrap()};

                debug!("REMOVED ITEM FROM QUEUE");
                unsafe {QUEUE.remove(index)};

                unsafe {PACKAGING_STATUS = JobStatus::Stopped;}
                update_frontend();
            } else {
                index += 1;
                debug!("Updating loop index")
            }
            debug!("LOOP DONE");
        }
        debug!("EXITED LOOP");
    });
}

fn generate_package_command(target_platform: String, build: String) -> (String, Vec<String>) {
    trace!("Generating package command");
    (Path::new(&get_config().ue_directory).join("Engine/Build/BatchFiles/RunUAT.bat").to_str().unwrap().to_string(),
    vec![String::from("BuildCookRun"),
            String::from(&format!("-project={}", &get_project_directory())),
            String::from(&format!("-targetplatform={}", target_platform)),
            String::from("-pak"),
            String::from("-unattended"),
            String::from("-prereqs"),
            String::from("-cook"),
            String::from("-stage"),
            String::from("-build"),
            String::from("-package"),
            String::from(&format!("-configuration={}", build)),
            String::from("-archive"),
            String::from(&format!("-archivedirectory={}", &get_compiled_output_directory()))
            ])
}

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    trace!("Hello, World! I'm awake!");
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
