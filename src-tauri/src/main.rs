// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod documents;
mod get_api_keys;
mod glossary;
mod process_call;
mod translate;
mod utils;
use ai_translator;

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
            let binding = app.path().resolve(".", BaseDirectory::Resource).unwrap();
            let path = binding.to_str().unwrap();
            env::set_var("PYTHONPATH", path);
            
            ai_translator::initialize_modules(&app);

            ai_translator::run_updater(&app);

            let _ = process_call::set_sys_path();

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
