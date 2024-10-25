pub mod glossary_handler {
    use crate::{lib, process_call};
    use process_call::handle_python_call;

    #[tauri::command]
    pub async fn get_glossaries(api_key: &str) -> Result<String, String> {
        handle_python_call(
            lib::get_glossary().unwrap_or(""),
            "glossary",
            "Glossario",
            Some(vec![api_key]),
            "get_glossaries",
            None,
            None,
        )
        .map_err(|e| e.to_string())
    }
}
