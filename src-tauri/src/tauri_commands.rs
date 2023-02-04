use log::{trace, info};
use tauri::api::dialog;

use crate::config::{get_config, set_config};
use crate::globals::{PROJECT_DIRECTORY,
    get_main_window,
    COMPILED_OUTPUT_DIRECTORY,
    update_frontend,
    Configuration,
    CLIENT_CONFIGURATION,
    SERVER_CONFIGURATION,
    SERVER_PACKAGING_STATUS,
    JobStatus,
    QUEUE,
    QueueElement,
    PACKAGING_STATUS
};
use crate::{run_queue, generate_package_command, stop_queue};

#[tauri::command]
pub fn get_ue_directory() -> String {
    trace!("Returning: {}", get_config().ue_directory);
    return get_config().ue_directory;
}

#[tauri::command]
pub fn set_ue_directory(new_directory: String) {
    trace!("Setting UE_DIRECTORY_LOCATION: {}", new_directory);

    let mut config = get_config();
    config.ue_directory = new_directory;
    set_config(config);

    update_frontend();
}

#[tauri::command]
pub fn set_is_source_directory(is_source: bool) {
    trace!("Setting ue_source: {}", is_source.to_string());

    let mut config = get_config();
    config.ue_source = is_source;
    set_config(config);

    update_frontend();
}

#[tauri::command]
pub fn get_is_source_directory() -> bool {
    let b = get_config().ue_source;
    trace!("get_is_source: {}", b);
    return b;
}

#[tauri::command]
pub fn get_project_directory() -> String {
    let project_directory = PROJECT_DIRECTORY.lock().unwrap().clone().unwrap_or(String::new());
    trace!("project_directory: {}", project_directory);
    return project_directory;
}

#[tauri::command]
pub fn set_project_directory(new_directory: String) {
    trace!("Setting PROJECT_DIRECTORY: {}", new_directory);

    *PROJECT_DIRECTORY.lock().unwrap() = Some(new_directory);
    update_frontend();
}

#[tauri::command]
pub fn open_project_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .add_filter("Unreal Engine Project File", &["uproject"])
          .pick_file(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    set_project_directory(path_buf.to_str().unwrap().to_string());
                },
                None => ()
            }
        );
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn open_ue_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .pick_folder(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    if path_buf.join("Engine/Build/BatchFiles/RunUAT.bat").is_file() {
                        set_ue_directory(path_buf.to_str().unwrap().to_string());
                        set_is_source_directory(!path_buf.join("Engine/Build/InstalledBuild.txt").is_file());
                    }
                    else {
                        dialog::message(Some(&get_main_window()), "Error", "The selected directory does not appear to be an Unreal Engine install!\n\nCanceling.");
                    }
                    
                },
                None => {}
            }
        );
}

#[tauri::command]
pub fn open_output_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .pick_folder(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    set_compiled_output_directory(path_buf.to_str().unwrap());
                },
                None => {}
            }
        );
}

#[tauri::command]
pub fn set_compiled_output_directory(new_directory: &str) {
    trace!("Setting COMPILED_OUTPUT_DIRECTORY: {}", new_directory);
    *COMPILED_OUTPUT_DIRECTORY.lock().unwrap() = Some(new_directory.to_string());
    update_frontend();
}

#[tauri::command]
pub fn get_compiled_output_directory() -> String {
    let output_directory = COMPILED_OUTPUT_DIRECTORY.lock().unwrap().clone().unwrap_or(String::new());
    trace!("COMPILED_OUTPUT_DIRECTORY: {}", serde_json::to_string_pretty(&output_directory).unwrap().to_string());
    return output_directory;
}

#[tauri::command]
pub fn set_client_configuration(new_client_config: Configuration) {
    info!("New client config: {}", serde_json::to_string_pretty(&new_client_config).unwrap().to_string());
    *CLIENT_CONFIGURATION.lock().unwrap() = new_client_config;
    update_frontend();
}

#[tauri::command]
pub fn get_client_configuration() -> Configuration {
    let returned_client_configuration = CLIENT_CONFIGURATION.lock().unwrap().clone();
    trace!("CLIENT_CONFIGURATION: {}", serde_json::to_string_pretty(&returned_client_configuration).unwrap().to_string());
    return returned_client_configuration;
    
}

#[tauri::command]
pub fn set_server_configuration(new_server_config: Configuration) {
    info!("New server config: {}", serde_json::to_string_pretty(&new_server_config).unwrap().to_string());
    *SERVER_CONFIGURATION.lock().unwrap() = new_server_config;
    update_frontend();
}

#[tauri::command]
pub fn get_server_configuration() -> Configuration {
    let returned_server_configuration = SERVER_CONFIGURATION.lock().unwrap().clone();
    trace!("SERVER_CONFIGURATION: {}", serde_json::to_string_pretty(&returned_server_configuration).unwrap().to_string());
    return returned_server_configuration;
}

#[tauri::command]
pub fn package_client() {
    trace!("package client pressed");
    let a = get_client_packaging_status();
    trace!("Status: {}", a);
    if a == JobStatus::Running
    {
        trace!("Stopping Queue!");
        stop_queue();
    }
    else {
        trace!("Packaging Client!");
        for build in &get_client_configuration().configuration {
            trace!("Building Queue");
            let (command, args) = generate_package_command(build.to_string(), get_client_configuration().build.to_string());
            let a = QueueElement { job_status: JobStatus::Waiting, command: command, args: args};
            trace!("Pushing value to queue");
            unsafe {QUEUE.push(a)};
            trace!("Done");
        }

        for el in unsafe {QUEUE.iter()} {
            println!("{}", serde_json::to_string_pretty(el).unwrap());
        }

        run_queue();
    }
    
}

#[tauri::command]
pub fn package_server() {
    trace!("Packaging Server!");
    for build in &get_server_configuration().configuration {
        let (command, args) = generate_package_command(get_client_configuration().build.to_string(), build.to_string());
        let a = QueueElement { job_status: JobStatus::Waiting, command: command, args: args};
        unsafe {QUEUE.push(a)};
    }
    run_queue();
}

#[tauri::command]
pub fn get_client_packaging_status() -> JobStatus {
    unsafe {PACKAGING_STATUS}
}

#[tauri::command]
pub fn get_server_packaging_status() -> JobStatus {
    SERVER_PACKAGING_STATUS.lock().unwrap().clone()
}