use ai_client::ChatGPT;
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