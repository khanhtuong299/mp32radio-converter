
use std::{path::Path, fs::File};
use lazy_static::lazy_static;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

struct GlobalState {
    song_name: String,
    path: String,
    is_playable:bool,
    is_coverted:bool,
    covert_name:String,
    state: PlayerState,
}
#[derive(Clone)] #[derive(Debug)]
enum PlayerState {
    Play,
    Pause,
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
        println!("tuong current state {:?}", state);
        self.state = state
    }
    fn get_state(&self)->PlayerState {
        self.state.clone()
    }
    
    fn get_file_path(&self) -> String {        
        Path::new(&(self.path)).to_str().unwrap().to_string()
    }
}

lazy_static!{
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState{
        song_name: String::from("tmp"),
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
    let curstate = PlayerState::Init;
    cur_state.state = curstate;
}

pub fn on_input(path: &str) -> &str {
    
    let file_path: &Path = Path::new(path);
    let file: BufReader<File> = BufReader::new(
        match File::open(file_path) {
            Ok(v) => v,
            Err(_) => {
                println!("File is corrupted!");
                return "File is corrupted!"
            }
        }
    );

    match Decoder::new(file) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid music file");
            return "Invalid music file"
        }
    };
    
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    cur_state.song_name = String::from(file_name);
    cur_state.path = String::from(path);
    cur_state.is_playable = true;
    cur_state.is_coverted = false;
    
    file_name
} 

pub fn reset_state(){
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    cur_state.song_name = "".to_string();
    cur_state.is_playable = false;
    cur_state.is_coverted = false;
    cur_state.covert_name = "".to_string();
    cur_state.path = String::from("./");

    match cur_state.get_state() {
        PlayerState::Init => (),
        _ => {
            let curstate = PlayerState::Reset;
            cur_state.set_state(curstate);
        }
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

pub fn on_play() -> bool{

    let audio_player = init_audio_player().lock().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();

    if let PlayerState::Pause = cur_state.get_state() {
        audio_player.sink.play();
        let curstate = PlayerState::Play;
        cur_state.set_state(curstate);
        return true;
    }

    let path: String = cur_state.get_file_path();
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

    
    let curstate = PlayerState::Play;
    cur_state.set_state(curstate);
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
            let curstate = PlayerState::Pause;
            cur_state.set_state(curstate);
        },
        _ => {}
    }
    true
}

pub fn on_stop() -> bool {
    let audio_player = init_audio_player().lock().unwrap();
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    audio_player.sink.stop();
    let curstate = PlayerState::Stop;
    cur_state.set_state(curstate);
    true
}