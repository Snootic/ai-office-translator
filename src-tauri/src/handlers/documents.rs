pub mod documents_handler {

    use core::str;

    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use translator::Document;

    pub fn copy_file(file_data: Vec<u8>, file_name: &str) -> Result<PathBuf, String> {
        let file_relative_path = format!(".{}", file_name);

        let mut file = File::create(&file_relative_path).map_err(|e| e.to_string())?;

        file.write_all(&file_data).map_err(|e| e.to_string())?;

        let file_path = PathBuf::from(&file_relative_path);
        let file_abs_path = std::fs::canonicalize(file_path).map_err(|e| e.to_string())?;

        Ok(file_abs_path)
    }

    #[tauri::command]
    pub fn load_document(file_data: Vec<u8>, file_name: &str) -> Result<Document, String> {
        let file_absolute_path: PathBuf = match copy_file(file_data, file_name) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error copying file: {}", e);
                return Err(e);
            }
        };

        let mut document = Document::new(file_absolute_path);
        document.load().map_err(|_| "Failed to load document".to_string())?;

        Ok(document) 
    }
}
