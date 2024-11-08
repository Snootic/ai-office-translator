pub mod utils_handler {
    use crate::{ai_translator, process_call};
    use process_call::handle_python_call;

    #[tauri::command]
    pub async fn get_gpt_models(api_key: &str) -> Result<String, String> {
        handle_python_call(
            ai_translator::get_utils().unwrap_or(""),
            "utils",
            "GPTAccount",
            Some(vec![api_key]),
            "models",
            None,
            None,
        )
        .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_source_languages(api_key: &str) -> Result<String, String> {
        handle_python_call(
            ai_translator::get_utils().unwrap_or(""),
            "utils",
            "DeeplAccount",
            Some(vec![api_key]),
            "get_languages",
            Some(vec!["source"]),
            None,
        )
        .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_target_languages(api_key: &str) -> Result<String, String> {
        handle_python_call(
            ai_translator::get_utils().unwrap_or(""),
            "utils",
            "DeeplAccount",
            Some(vec![api_key]),
            "get_languages",
            Some(vec!["target"]),
            None,
        )
        .map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn check_usage(api_key: &str) -> Result<String, String> {
        handle_python_call(
            ai_translator::get_utils().unwrap_or(""),
            "utils",
            "DeeplAccount",
            Some(vec![api_key]),
            "check_usage",
            None,
            None,
        )
        .map_err(|e| e.to_string())
    }
}
