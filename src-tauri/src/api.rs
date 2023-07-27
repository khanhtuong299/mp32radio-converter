#[path = "music_processing.rs"] mod music_processing;

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
  music_processing::reset_state();
  music_processing::on_input(path)
}

#[tauri::command]
pub fn play_music2() -> bool {
    music_processing::on_play()
}

#[tauri::command]
pub fn pause_music() -> bool {
    music_processing::on_pause()
}

#[tauri::command]
pub fn stop_music() -> bool {
    music_processing::on_stop()
}

pub fn init_processing(){
  music_processing::init_processing()
}