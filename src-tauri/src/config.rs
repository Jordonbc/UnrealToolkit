use std::fs;
use crate::global_variables::{CONFIG, CONFIG_MUTEX, ConfigTemplate ,CONFIG_LOCATION};

pub fn get_config() -> ConfigTemplate {
    let _guard = CONFIG_MUTEX.lock().unwrap();
    unsafe {
        return CONFIG.clone().unwrap();
    }
}

pub fn set_config(new_setting: ConfigTemplate) {
    unsafe {
        let _guard = CONFIG_MUTEX.lock().unwrap();
        CONFIG = Some(new_setting);
    }
    save_config_file();
}

pub fn reset_config() {
    set_config(ConfigTemplate { ue_directory: String::new() });
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error Creating file!");
}

fn try_create_config_folders() {
    if !CONFIG_LOCATION.exists() {
        let directory_tree = CONFIG_LOCATION.as_path().clone().parent().unwrap();

        println!("Creating folder tree: {}", directory_tree.to_str().unwrap());
        fs::create_dir_all(directory_tree).expect("failed to create folders");
    }
}

pub fn read_config_file() {
    println!("Reading config file: {}", CONFIG_LOCATION.to_str().unwrap());

    try_create_config_folders();

    let config_result = fs::read_to_string(CONFIG_LOCATION.to_str().unwrap());
    match config_result {
        Ok(_) => {
            let sanitized_config = serde_json::from_str(config_result.unwrap().as_str());
            match sanitized_config {
                Ok(_) => {
                    set_config(sanitized_config.unwrap());
                },
                Err(_) => reset_config(),
            }
        },
        Err(_) => reset_config(),
    }
}

fn save_config_file() {
    println!("Saving config file: {}", CONFIG_LOCATION.to_str().unwrap());
    try_create_config_folders();
    fs::write(CONFIG_LOCATION.to_str().unwrap(), serde_json::to_string_pretty(&get_config()).unwrap()).expect("Error saving file!");
}