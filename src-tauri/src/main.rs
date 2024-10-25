// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod documents;
mod get_api_keys;
mod glossary;
mod process_call;
mod translate;
mod utils;
mod lib;

use std::env;

use documents::documents_handler;
use get_api_keys::{get_deepl_keys, get_gpt_keys, Item};
use glossary::glossary_handler;
use tauri::{path::BaseDirectory, Manager};
use translate::translate_handler;
use utils::utils_handler;

#[tauri::command]
fn get_chatgpt_keys() -> Result<Vec<Item>, String> {
    match crate::get_gpt_keys() {
        Ok(keys) => Ok(keys),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_deep_keys() -> Result<Vec<Item>, String> {
    match crate::get_deepl_keys() {
        Ok(keys) => Ok(keys),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .setup(|app| {
            let binding = app.path().resolve("src/translator/documents.py", BaseDirectory::Resource)?;
            let doc = binding.to_str().unwrap();

            let binding = app.path().resolve("src/.venv", BaseDirectory::Resource)?;
            let venv_path = binding.to_str().unwrap();

            let binding = app.path().resolve("src/.venv/bin", BaseDirectory::Resource)?;
            let bin_path = binding.to_str().unwrap();

            let cur_path = env::var("PATH").unwrap();

            let new_path = format!("{}:{}:{}", bin_path, venv_path, cur_path);

            env::set_var("PATH", new_path);
            
            env::remove_var( "PYTHONHOME");
            
            lib::initialize_modules(&app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            documents_handler::load_document,
            glossary_handler::get_glossaries,
            utils_handler::get_gpt_models,
            utils_handler::get_source_languages,
            utils_handler::get_target_languages,
            utils_handler::check_usage,
            translate_handler::translate_document,
            get_chatgpt_keys,
            get_deep_keys
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
