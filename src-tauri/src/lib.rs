use std::sync::Once;

use tauri_plugin_updater::UpdaterExt;
use tauri::{path::BaseDirectory, App, Manager};

static INIT: Once = Once::new();
static mut DOCUMENTS: Option<String> = None;
static mut UTILS: Option<String> = None;
static mut TRANSLATE: Option<String> = None;
static mut GLOSSARY: Option<String> = None;
static mut GPT_KEYS: Option<String> = None;
static mut DEEPL_KEYS: Option<String> = None;

pub fn initialize_modules(app: &App) {
    INIT.call_once(|| {
        let binding = app.path().resolve("src/translator/documents.py", BaseDirectory::Resource).unwrap();
        unsafe {
            DOCUMENTS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app.path().resolve("src/translator/utils.py", BaseDirectory::Resource).unwrap();
        unsafe {
            UTILS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app.path().resolve("src/translator/translate.py", BaseDirectory::Resource).unwrap();
        unsafe {
            TRANSLATE = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app.path().resolve("src/translator/glossary.py", BaseDirectory::Resource).unwrap();
        unsafe {
            GLOSSARY = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app.path().resolve("src/config/gpt_keys.json", BaseDirectory::Resource).unwrap();
        unsafe {
            GPT_KEYS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app.path().resolve("src/config/deepl_keys.json", BaseDirectory::Resource).unwrap();
        unsafe {
            DEEPL_KEYS = Some(binding.to_str().unwrap().to_string());
        }
    });
}

pub fn get_documents() -> Option<&'static str> {
    unsafe { DOCUMENTS.as_deref() }
}

pub fn get_utils() -> Option<&'static str> {
    unsafe { UTILS.as_deref() }
}

pub fn get_translate() -> Option<&'static str> {
    unsafe { TRANSLATE.as_deref() }
}

pub fn get_glossary() -> Option<&'static str> {
    unsafe { GLOSSARY.as_deref() }
}

pub fn get_gpt_keys_path() -> Option<&'static str> {
    unsafe { GPT_KEYS.as_deref() }
}

pub fn get_deepl_keys_path() -> Option<&'static str> {
    unsafe { DEEPL_KEYS.as_deref() }
}
