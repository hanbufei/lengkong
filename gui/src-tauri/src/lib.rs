use service::class::{add_class, delete_class, edit_class, get_class, list_class};
use service::label::{add_label, delete_label, edit_label, get_label, list_label};
use service::article::{add_article, delete_article, edit_article_text, get_article, list_article};

mod dao;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    // .plugin( /* Add your Tauri plugin here */ )
    // Add your commands here that you will call from your JS code  
    .invoke_handler(tauri::generate_handler![
      //class服务
      get_class,
      list_class,
      add_class,
      delete_class,
      edit_class,
      //label服务
      get_label,
      list_label,
      add_label,
      delete_label,
      edit_label,
      //article服务
      get_article,
      list_article,
      add_article,
      delete_article,
      edit_article_text,
      ])
    .plugin(tauri_plugin_notification::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
