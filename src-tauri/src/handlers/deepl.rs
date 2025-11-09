use crate::models::deepl::{Deepl, Glossary};
use serde_json::Value;

#[tauri::command]
pub async fn check_usage(api_key: &str) -> Result<Value, String> {
    let deepl: Deepl = Deepl::new(String::from(api_key));
    match deepl.check_usage().await {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_source_languages(api_key: &str) -> Result<Value, String> {
    let deepl: Deepl = Deepl::new(String::from(api_key));
    match deepl.get_source_languages().await {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_target_languages(api_key: &str) -> Result<Value, String> {
    let deepl: Deepl = Deepl::new(String::from(api_key));
    match deepl.get_target_languages().await {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_glossary_from_excel(api_key: &str, excel_file_path: String) -> Result<Value, String> {
    let deepl: Deepl = Deepl::new(String::from(api_key));
    Ok(deepl.create_glossary_from_excel(excel_file_path).await)
}

#[tauri::command]
pub async fn get_glossaries(api_key: &str) -> Result<Value, String> {
    let deepl: Deepl = Deepl::new(String::from(api_key));
    Ok(deepl.get_glossaries().await)
}