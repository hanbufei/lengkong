use service::class::get_class_by_id;

mod dao;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    // .plugin( /* Add your Tauri plugin here */ )
    // Add your commands here that you will call from your JS code  
    .invoke_handler(tauri::generate_handler![get_class_by_id])
    .plugin(tauri_plugin_notification::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
