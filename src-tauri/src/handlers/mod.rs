use tauri::generate_handler;

mod deepl;
mod chatgpt;
mod get_api_keys;
mod translate;
pub mod documents;

use documents::documents_handler;
use translate::translate_handler;

pub fn handlers<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool {
    generate_handler![
      get_api_keys::get_gpt_keys,
      get_api_keys::get_deepl_keys,
      chatgpt::get_gpt_models,
      chatgpt::get_gpt_billing,
      deepl::check_usage,
      deepl::get_source_languages,
      deepl::get_target_languages,
      deepl::create_glossary,
      deepl::get_glossaries,
      deepl::delete_glossary,
      documents_handler::load_document,
      translate_handler::translate_document,
    ]
}