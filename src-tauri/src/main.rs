// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod process_call;
mod models;
mod handlers;
mod structs;

use ai_translator;

use std::{env, sync::Mutex};

use tauri::Manager;

use serde_json::Value;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .manage(Mutex::new(ai_translator::SideTasks {
            updater: false,
            dependencies: false,
        }))
        .setup(|app| {
            let data_dir = app.path().app_data_dir().unwrap();
            let initial_config_json_path = data_dir.join("windows_initial_config.json");

            if cfg!(target_os = "windows") {
                let initial_config_content = match std::fs::read_to_string(&initial_config_json_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Error reading windows initial config: {}", e);
                        String::new()
                    }
                };
                let initial_config_json: Value = match serde_json::from_str(&initial_config_content) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Error parsing windows initial config: {}", e);
                        Value::Object(serde_json::Map::new())
                    }
                };

                if !initial_config_json_path.exists() || initial_config_json["initial_config"] == true {
                    println!("Running initial config for windows");
                    let _ = ai_translator::windows_initial_config(app);
                    return Ok(());
                }
            }
            
            let libs_binding = cfg!(target_os = "windows")
                .then(|| data_dir.clone())
                .unwrap_or(data_dir.join("Python311"));
            let lib_path = libs_binding.to_str().unwrap();

            let python_path_binding = cfg!(target_os = "windows")
                .then(|| libs_binding.join("Python311"))
                .unwrap_or(libs_binding.join("python3.11"));
            let python_path = python_path_binding.to_str().unwrap();

            let sys_path = env::var("PATH").unwrap_or_default();

            let bin_binding = cfg!(target_os = "windows")
                .then(|| libs_binding.join("Python311\\Scripts"))
                .unwrap_or(libs_binding.join("bin"));
            let bin_path = bin_binding.to_str().unwrap();

            let path = cfg!(target_os = "windows")
                .then(|| format!("{};{};{}", sys_path, lib_path, bin_path))
                .unwrap_or(format!("{}:{}:{}", sys_path, lib_path, bin_path));

            env::set_var("PATH", path);
            env::set_var("PYTHONPATH", python_path_binding.join("Python311.zip").to_str().unwrap());
            env::set_var("PYTHONUSERBASE", lib_path);

            if cfg!(target_os = "windows") {
                env::set_var("PYTHONHOME", python_path);
            }
            
            ai_translator::initialize_modules(&app);
            
            let _ = process_call::set_sys_path(libs_binding);
            
            ai_translator::run_updater(app);

            ai_translator::handle_dependencies(app);
            
            Ok(())
        })
        .invoke_handler(handlers::handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
