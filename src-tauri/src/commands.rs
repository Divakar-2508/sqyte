use std::sync::Mutex;

use tauri::State;

use crate::models::{AppState, Table};

#[tauri::command]
pub fn get_tables(app_state: State<Mutex<AppState>>) -> Result<Vec<Table>, String> {
    let app_state = app_state.lock().unwrap();
    Table::get_tables(&app_state.db).map_err(|err| err.to_string())
}