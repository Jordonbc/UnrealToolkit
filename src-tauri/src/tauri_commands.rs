use tauri::api::dialog;
use tauri::Manager;

use crate::config::{get_config, set_config};
use crate::global_variables::{PROJECT_DIRECTORY, get_main_window};

#[tauri::command]
pub fn get_ue_directory() -> String {
    get_config().ue_directory;
    print!("Returning: {}", get_config().ue_directory);

    return get_config().ue_directory;
}

#[tauri::command]
pub fn set_ue_directory(new_directory: String) {
    println!("Setting UE_DIRECTORY_LOCATION: {}", new_directory);
    let mut config = get_config();
    config.ue_directory = new_directory;
    set_config(config);
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
}

#[tauri::command]
pub async fn open_project_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .add_filter("Unreal Engine Project File", &["uproject"])
          .pick_file(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    set_project_directory(path_buf.to_str().unwrap().to_string());
                    get_main_window().emit_all("project_directory_changed", get_project_directory()).expect("Error Sending directory changed event to frontend!");
                },
                None => ()
            }
        );
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn open_ue_directory_dialog() {
    dialog::FileDialogBuilder::default()
          .pick_folder(|path_buf| 
            match path_buf {
                Some(path_buf) => {
                    if path_buf.join("Engine/Build/BatchFiles/RunUAT.bat").is_file() {
                        set_ue_directory(path_buf.to_str().unwrap().to_string());
                        get_main_window().emit_all("ue_directory_changed", get_ue_directory()).expect("Error Sending directory changed event to frontend!");
                    }
                    else {
                        dialog::message(Some(&get_main_window()), "Error", "The selected directory does not appear to be an Unreal Engine install!\n\nCanceling.");
                    }
                    
                },
                None => {}
            }
        );
}