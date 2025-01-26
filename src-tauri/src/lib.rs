use std::{process::Stdio, sync::{Mutex, Once}};

use serde::Serialize;
use tauri::{path::BaseDirectory, App, Emitter, Manager, State};
use tauri_plugin_updater::UpdaterExt;
use tokio::io::{ BufReader, AsyncBufReadExt };

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

pub struct SideTasks {
    pub updater: bool,
    pub dependencies: bool,
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

#[tauri::command]
async fn set_complete(app: tauri::AppHandle, state: State<'_, Mutex<SideTasks>>, task: String) -> Result<(), ()> {
  let mut state = state.lock().unwrap();

    match task.as_str() {
        "updater" => state.updater = true,
        "dependencies" => state.dependencies = true,
        _ => (),
    }

    if state.updater && state.dependencies {
        let update_window = app.get_webview_window("update").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        update_window.close().unwrap();
        main_window.show().unwrap();
    }
  
  Ok(())
}

pub fn run_updater(app: &App) {
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
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

        set_complete(
            app.clone(), 
            app.state::<Mutex<SideTasks>>(),
            "updater".to_string()
        ).await.unwrap();
    }

    Ok(())
}

pub fn handle_dependencies(app: &App) {
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let _ = install_dependencies(handle).await;
    });
}

async fn install_dependencies(app: tauri::AppHandle) -> Result<(), ()> {
    let mut binding = app.path().app_data_dir().unwrap();
    println!("installing dependencies");
    binding = binding.join("lib");
    let path = binding.to_str().unwrap();
    
    let mut cmd = tokio::process::Command::new("pip");

    cmd.args(&["install", "-r", "https://raw.githubusercontent.com/snootic/ai-office-translator/main/requirements.txt", "--target", path]);
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let stdout = child.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child.wait().await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await.map_err(|_| ())? {
        println!("Line: {}", line);
    }

    set_complete(
        app.clone(), 
        app.state::<Mutex<SideTasks>>(),
        "dependencies".to_string()
    ).await?;

    Ok(())
}
