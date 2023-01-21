use tauri::api::dialog;

use crate::config::{get_config, set_config};
use crate::globals::{PROJECT_DIRECTORY, get_main_window, COMPILED_OUTPUT_DIRECTORY, CONFIG_MUTEX, update_frontend};

#[tauri::command]
pub fn get_ue_directory() -> String {
    print!("Returning: {}", get_config().ue_directory);

    return get_config().ue_directory;
}

#[tauri::command]
pub fn set_ue_directory(new_directory: String) {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    println!("Setting UE_DIRECTORY_LOCATION: {}", new_directory);
    let mut config = get_config();
    config.ue_directory = new_directory;
    set_config(config);
    update_frontend();
}

#[tauri::command]
pub fn set_is_source_directory(is_source: bool) {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    println!("Setting ue_source: {}", is_source.to_string());
    let mut config = get_config();
    config.ue_source = is_source;
    set_config(config);
    update_frontend();
}

#[tauri::command]
pub fn get_is_source_directory() -> bool {
    return get_config().ue_source;
}

#[tauri::command]
pub fn get_project_directory() -> String {
    unsafe {
        let p = PROJECT_DIRECTORY.clone().unwrap_or(String::new());
        return p;
    }
}

#[tauri::command]
pub fn set_project_directory(new_directory: String) {
    println!("Setting PROJECT_DIRECTORY: {}", new_directory);
    unsafe {
        PROJECT_DIRECTORY = Some(new_directory);
    }
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
    println!("Setting COMPILED_OUTPUT_DIRECTORY: {}", new_directory);
    unsafe {
        COMPILED_OUTPUT_DIRECTORY = Some(new_directory.to_string());
    }
    update_frontend();
}

#[tauri::command]
pub fn get_compiled_output_directory() -> String {
    unsafe {
        COMPILED_OUTPUT_DIRECTORY.clone().unwrap()
    }
}