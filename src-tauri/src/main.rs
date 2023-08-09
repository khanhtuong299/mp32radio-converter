// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;

fn main() {
  api::init_processing();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      api::greet
      ,api::on_drop
      ,api::play_music2
      ,api::pause_music
      ,api::stop_music
      ,api::convert2radio
      ,api::on_radio
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

