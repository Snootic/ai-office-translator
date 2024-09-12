pub mod utils_handler {
    use crate::process_call;
    use process_call::handle_python_call;

    #[tauri::command]
    pub async fn get_gpt_models (api_key: &str) -> Result<String, String> {
        handle_python_call(
            "../src/translator/utils.py", 
            "utils", 
            "gpt",
            Some(vec![api_key]), 
            "models", 
            None, 
            None
        ).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_source_languages (api_key: &str) -> Result<String, String> {
        handle_python_call(
            "../src/translator/utils.py", 
            "utils", 
            "deep",
            Some(vec![api_key]), 
            "get_languages", 
            Some(vec!["source"]), 
            None
        ).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_target_languages (api_key: &str) -> Result<String, String> {
        handle_python_call(
            "../src/translator/utils.py", 
            "utils", 
            "deep",
            Some(vec![api_key]), 
            "get_languages", 
            Some(vec!["target"]), 
            None
        ).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn check_usage (api_key: &str) -> Result<String, String> {
        handle_python_call(
            "../src/translator/utils.py", 
            "utils", 
            "deep",
            Some(vec![api_key]), 
            "check_usage", 
            None, 
            None
        ).map_err(|e| e.to_string())
    }
}