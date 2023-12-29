// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;

fn main() {
  tracing_subscriber::fmt::init();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![set_window_decorations])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn set_window_decorations(window: tauri::Window, decorations: bool) {
    info!("Window: {}", window.label());
    window.set_decorations(decorations).unwrap();
}