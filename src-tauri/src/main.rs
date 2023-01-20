#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::{env};
use tauri::{Manager, Size, LogicalSize};

use crate::global_variables::*;
mod global_variables;
use crate::tauri_commands::*;
mod tauri_commands;
use crate::config::*;
mod config;

fn main() {
    {
        read_config_file();
        //let config = ConfigTemplate { ue_directory: String::new()};
        //set_config(config);
    }
    
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
            get_is_source_directory
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
