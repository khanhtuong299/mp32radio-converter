#[path = "music_2_radio.rs"] mod music2radio;

use std::{path::Path, fs::File};
use lazy_static::lazy_static;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

struct GlobalState {
    song_name: String,
    playing_name: String,
    radio_path: String,
    path: String,
    is_playable:bool,
    is_coverted:bool,
    covert_name:String,
    state: PlayerState,
}
#[derive(Clone)] #[derive(Debug)]
enum PlayerState {
    Play,
    PlayRadio,
    Pause,
    PauseRadio,
    Stop,
    Reset,
    Init
}
struct AudioPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl GlobalState {
    fn set_state(&mut self, state:PlayerState) {
        self.state = state
    }
    fn get_state(&self)->PlayerState {
        self.state.clone()
    }
    
    fn get_file_path(&self) -> String {        
        Path::new(&(self.path)).to_str().unwrap().to_string()
    }

    fn get_song_name(&self) -> String {
        self.song_name.clone()
    }
}

lazy_static!{
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState{
        song_name: String::from("tmp"),
        playing_name: String::from("tmp"),
        radio_path: String::from("tmp"),
        path: String::from("./"),
        is_playable:false,
        is_coverted:false,
        covert_name:String::from("tmp_converted"),
        state:PlayerState::Init,
    });
}

pub fn init_processing(){
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    cur_state.song_name = "".to_string();
    cur_state.is_playable = false;
    cur_state.is_coverted = false;
    cur_state.covert_name = "".to_string();
    cur_state.path = String::from("./");
    cur_state.set_state(PlayerState::Init);
}

pub fn on_input(path: &str) -> String {
    
    let file_path: &Path = Path::new(path);
    let file: BufReader<File> = BufReader::new(
        match File::open(file_path) {
            Ok(v) => v,
            Err(_) => {
                println!("File is corrupted!");
                return "File is corrupted!".to_string()
            }
        }
    );

    match Decoder::new(file) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid music file");
            return "Invalid music file".to_string()
        }
    };
    
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();

    if *path == cur_state.get_file_path() {
        let audio_player = init_audio_player().lock().unwrap();

        if audio_player.sink.is_paused() {
            cur_state.set_state(PlayerState::Pause);
        } else {
            cur_state.set_state(PlayerState::Play);
        }

        return String::from(file_name);
    } 
    
    cur_state.song_name = String::from(file_name);
    cur_state.path = String::from(path);
    cur_state.is_playable = true;
    cur_state.is_coverted = false;

    String::from(file_name)

    // if cur_state.playing_name == "tmp".to_string() {
    //     return String::from(file_name);
    // }

    // format!("Pending: {}\nPlaying: {}", String::from(file_name), cur_state.playing_name)
    
    
} 

pub fn reset_state(){
    let mut cur_state = GLOBAL_STATE.lock().unwrap();

    match cur_state.get_state() {
        PlayerState::Init => (),
        _ => cur_state.set_state(PlayerState::Reset),
    }
}



fn init_audio_player() -> &'static Mutex<AudioPlayer> {
    static mut CONF: MaybeUninit<Mutex<AudioPlayer>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(
            AudioPlayer {
                _stream:stream,
                _stream_handle:stream_handle.clone(),
                sink:Sink::try_new(&stream_handle).unwrap(),
            }
        ));
    });
    unsafe { &*CONF.as_ptr() }
}

// fn get_sink()-> Sink {
//     let audio_player = init_audio_player().lock().unwrap();
//     let sink = Sink::try_new(&audio_player.stream_handle).unwrap();
//     sink
// }

pub fn on_play() -> bool {
    on_play_iml(false)
}

pub fn on_play_radio() -> bool {
    on_play_iml(true)
}

fn on_play_iml(radio:bool) -> bool{

    let audio_player = init_audio_player().lock().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();

    match cur_state.get_state() {
        PlayerState::Pause => {
            audio_player.sink.play();
            if radio {
                
                audio_player.sink.stop();
            } else {
                cur_state.set_state(PlayerState::Play);
                return true;
            }
        },
        PlayerState::PauseRadio => {
            audio_player.sink.play();
            if !radio {
                audio_player.sink.stop();
            } else {
                audio_player.sink.play();
                cur_state.set_state(PlayerState::PlayRadio);
                return true;
            }
        },
        PlayerState::PlayRadio => {
            if radio {
                return true;
            } else {
                audio_player.sink.stop();
            }
        },
        PlayerState::Play => {
            if !radio {
                return true;
            } else {
                audio_player.sink.stop();
            }
        },
        _ => {}
    }

    let path: String = if radio {
        cur_state.radio_path.clone()
    } else {
        cur_state.get_file_path()
    };
    
    let file_path: &Path = Path::new(path.as_str());

    let file: BufReader<File> = BufReader::new(
        match File::open(file_path) {
            Ok(v) => v,
            Err(_) => {
                println!("can not read file");
                return false;
            }
        }
    );
    let source = match Decoder::new(file) {
        Ok(v) => v,
        Err(_) => {
            println!("can not create source");
            return false;
        }
    };

    if let PlayerState::Reset = cur_state.get_state() {
        if audio_player.sink.is_paused() {
            audio_player.sink.play();
        }
        audio_player.sink.stop();
    }
    
    audio_player.sink.append(source);
    
    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    cur_state.playing_name = String::from(file_name);
    if radio {
        cur_state.set_state(PlayerState::PlayRadio);
    } else {
        cur_state.set_state(PlayerState::Play);
    }
    
    true
}

pub fn on_pause() -> bool{
    let audio_player = init_audio_player().lock().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();

    match cur_state.get_state() {
        PlayerState::Reset =>{ 
            if !audio_player.sink.is_paused() {
                audio_player.sink.pause();
            }
        },
        PlayerState::Play => {
            audio_player.sink.pause();
            cur_state.set_state(PlayerState::Pause);
        },
        PlayerState::PlayRadio => {
            audio_player.sink.pause();
            cur_state.set_state(PlayerState::PauseRadio);
        },
        _ => {}
    }
    true
}

pub fn on_stop() -> bool {
    let audio_player = init_audio_player().lock().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    if let PlayerState::Pause = cur_state.get_state() {
        audio_player.sink.play();
        audio_player.sink.stop();
    }
    audio_player.sink.stop();
    cur_state.set_state(PlayerState::Stop);
    true
}

pub fn convert_2_radio(){
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    let song_name = cur_state.get_song_name();
    let file_path = cur_state.get_file_path();
    music2radio::to_radio(&file_path, song_name.clone());
    let song_radio = format!("../radio_out/radio_{}.wav", song_name);
    cur_state.radio_path = song_radio;
}