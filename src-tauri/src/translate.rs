pub mod translate_handler {
    use crate::{documents::documents_handler::copy_file, lib, process_call};
    use process_call::handle_python_call;

    #[tauri::command]
    pub async fn translate_document(
        api_key: &str,
        file_data: Vec<u8>,
        file_name: &str,
        model: &str,
        mut args: Vec<&str>,
        kwargs: Option<Vec<(&str, &str)>>,
    ) -> Result<String, String> {
        let file_abs_path =
            copy_file(file_data, file_name).map_err(|e| format!("Failed to copy file: {}", e))?;

        args.insert(0, &file_abs_path);

        handle_python_call(
            lib::get_translate().unwrap_or(""),
            "translate",
            "Translate",
            Some(vec![api_key, model]),
            "translate_document",
            Some(args),
            kwargs,
        )
        .map_err(|e| e.to_string())
    }
}
