pub mod documents_handler {

    use core::str;

    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use crate::{ai_translator, process_call};
    use process_call::handle_python_call;

    pub fn copy_file(file_data: Vec<u8>, file_name: &str) -> Result<String, String> {
        let file_relative_path = format!(".{}", file_name);

        let mut file = File::create(&file_relative_path).map_err(|e| e.to_string())?;

        file.write_all(&file_data).map_err(|e| e.to_string())?;

        let file_path = PathBuf::from(&file_relative_path);
        let file_abs_path = std::fs::canonicalize(file_path).map_err(|e| e.to_string())?;

        Ok(file_abs_path.display().to_string())
    }

    #[tauri::command]
    pub fn load_document(file_data: Vec<u8>, file_name: &str) -> Result<String, String> {
        let file_absolute_path = match copy_file(file_data, file_name) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error copying file: {}", e);
                return Err(e);
            }
        };

        let args: Vec<&str> = vec![file_absolute_path.as_str()];

        handle_python_call(
            ai_translator::get_documents().unwrap_or(""),
            "documents",
            "File",
            None,
            "load_document",
            Some(args),
            None,
        )
        .map_err(|e| e.to_string())
        
    }
}
