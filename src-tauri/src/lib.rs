use std::sync::Once;

use serde::Serialize;
use tauri::{path::BaseDirectory, App, Emitter, Manager};
use tauri_plugin_updater::UpdaterExt;

static INIT: Once = Once::new();
static mut DOCUMENTS: Option<String> = None;
static mut UTILS: Option<String> = None;
static mut TRANSLATE: Option<String> = None;
static mut GLOSSARY: Option<String> = None;
static mut GPT_KEYS: Option<String> = None;
static mut DEEPL_KEYS: Option<String> = None;

#[derive(Clone, Serialize)]
struct AppUpdate {
    total_size: Option<u64>,
    downloaded_size: usize,
    version: String,
}

pub fn initialize_modules(app: &App) {
    INIT.call_once(|| {
        let binding = app
            .path()
            .resolve("src/translator/documents.py", BaseDirectory::Resource)
            .unwrap();
        unsafe {
            DOCUMENTS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app
            .path()
            .resolve("src/translator/utils.py", BaseDirectory::Resource)
            .unwrap();
        unsafe {
            UTILS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app
            .path()
            .resolve("src/translator/translate.py", BaseDirectory::Resource)
            .unwrap();
        unsafe {
            TRANSLATE = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app
            .path()
            .resolve("src/translator/glossary.py", BaseDirectory::Resource)
            .unwrap();
        unsafe {
            GLOSSARY = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app
            .path()
            .resolve("src/config/gpt_keys.json", BaseDirectory::Resource)
            .unwrap();
        unsafe {
            GPT_KEYS = Some(binding.to_str().unwrap().to_string());
        }

        let binding = app
            .path()
            .resolve("src/config/deepl_keys.json", BaseDirectory::Resource)
            .unwrap();
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

pub fn run_updater(app: &App) {
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        println!("running");
        let _ = update(handle).await;
    });
}
async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                    app.emit(
                        "update-progress",
                        AppUpdate {
                            total_size: content_length,
                            downloaded_size: downloaded,
                            version: update.version.clone(),
                        },
                    )
                    .unwrap();
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    } else {
        println!("no update available");

        let update_window = app.get_webview_window("update").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        update_window.close().unwrap();
        main_window.show().unwrap();
    }

    Ok(())
}
