use serde::{Deserialize, Serialize};
use tauri::State;
use std::fs::File;
use std::io::BufReader;

use crate::ai_translator;

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub key: String,
}

#[tauri::command]
pub fn get_deepl_keys(paths: State<'_, ai_translator::AppPaths>) -> Result<Vec<Item>, String> {
    let file = File::open(paths.deepl_keys.clone())
        .map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let items: Vec<Item> = serde_json::from_reader(reader)
        .map_err(|e| e.to_string())?;

    Ok(items)
}

#[tauri::command]
pub fn get_gpt_keys(paths: State<'_, ai_translator::AppPaths>) -> Result<Vec<Item>, String> {
    let file = File::open(paths.gpt_keys.clone())
        .map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let items: Vec<Item> = serde_json::from_reader(reader)
        .map_err(|e| e.to_string())?;

    Ok(items)
}
