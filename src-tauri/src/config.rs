use std::fs;
use log::{info, trace, error};

use crate::globals::{CONFIG, ConfigTemplate ,CONFIG_LOCATION};

pub fn get_config() -> ConfigTemplate {
    return CONFIG.lock().unwrap().clone().unwrap();
}

pub fn set_config(new_setting: ConfigTemplate) {
    info!("Saving new configTemplate {}", serde_json::to_string_pretty(&new_setting).unwrap().to_string());
    *CONFIG.lock().unwrap() = Some(new_setting);
    save_config_file();
}

pub fn reset_config() {
    info!("Resetting config file!");
    set_config(ConfigTemplate { ue_directory: String::new(), ue_source: false });
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error Creating file!");
}

fn try_create_config_folders() {
    if !CONFIG_LOCATION.exists() {
        info!("Creating folder structure for config file");
        let directory_tree = CONFIG_LOCATION.as_path().clone().parent().unwrap();

        trace!("Creating folder tree: {}", directory_tree.to_str().unwrap());
        fs::create_dir_all(directory_tree).expect("failed to create folders");
    }
}

pub fn read_config_file() {
    info!("Reading config file: {}", CONFIG_LOCATION.to_str().unwrap());

    try_create_config_folders();

    let config_result = fs::read_to_string(CONFIG_LOCATION.to_str().unwrap());
    match config_result {
        Ok(_) => {
            let sanitized_config = serde_json::from_str(config_result.unwrap().as_str());
            info!("Successfully read config from file");
            match sanitized_config {
                Ok(_) => {
                    info!("Setting config");
                    set_config(sanitized_config.unwrap());
                },
                Err(_) => {
                    error!("Unable to parse config file, replacing!");
                    reset_config();
                },
            }
        },
        Err(_) => {
            error!("Unable to read config file, replacing!");
            reset_config()
        },
    }
}

fn save_config_file() {
    info!("Saving config file: {}", CONFIG_LOCATION.to_str().unwrap());
    try_create_config_folders();
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error saving file!");
}