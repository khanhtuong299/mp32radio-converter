#[path = "music_control.rs"] mod music_control;

#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("received command {}", name);
  format!("Hello, {}!", name)
}

// #[tauri::command]
// pub fn log(level: i32, content: &str){
//   println!("{} level: {}, msg: {}", chrono::offset::Local::now(),
//     level,
//     content);
// }

#[tauri::command]
pub fn on_drop(path: &str) -> String {
  music_control::reset_state();
  music_control::on_input(path)
}

#[tauri::command]
pub fn play_music2() -> bool {
    music_control::on_play()
}

#[tauri::command]
pub fn pause_music() -> bool {
    music_control::on_pause()
}

#[tauri::command]
pub fn stop_music() -> bool {
    music_control::on_stop()
}

#[tauri::command]
pub fn convert2radio() -> bool {
    music_control::convert_2_radio();
    true
}

#[tauri::command]
pub fn on_radio() -> bool {
    music_control::on_play_radio()
}


pub fn init_processing(){
  music_control::init_processing()
}