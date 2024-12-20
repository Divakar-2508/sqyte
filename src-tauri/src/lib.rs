mod models;
mod commands;

use std::sync::Mutex;

use commands::get_tables;
use tauri::Manager;
use rusqlite::Connection;
use models::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db = Connection::open("baka.db").unwrap();
            let app_state = AppState {
                db
            };
            app.manage(Mutex::new(app_state));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_tables])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
