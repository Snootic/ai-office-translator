pub mod utils_handler {
    use crate::{models::{chatgpt::ChatGPT, deepl::Deepl}};
    use serde_json::Value;

    #[tauri::command]
    pub async fn get_gpt_models(api_key: &str) -> Result<Vec<String>, String> {
        let chat_gpt = ChatGPT::new(String::from(api_key));
        match chat_gpt.list_models().await {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }

    #[tauri::command]
    pub async fn get_gpt_billing(api_key: &str) -> Result<Value, String> {
        let chat_gpt = ChatGPT::new(String::from(api_key));
        match chat_gpt.get_monthly_billing().await {
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
    pub async fn check_usage(api_key: &str) -> Result<Value, String> {
        let deepl: Deepl = Deepl::new(String::from(api_key));
        match deepl.check_usage().await {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }
}
