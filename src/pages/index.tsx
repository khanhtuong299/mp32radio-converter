import styles from '@/styles/style.module.css'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

enum LogLevel {
  no_log,
  debug,
  info,
  warn,
  error
}

enum MusicState {
  play,
  playradio,
  pause,
  stop,
}

let CurrentState: MusicState = MusicState.stop;

listen('tauri://file-drop', event => {
  let file_path = String(event.payload);

  invoke<string>('on_drop', { path: file_path })
    .then((response) => {
      let button = document.getElementById("toggle");
      button!.innerHTML = "&#9835&#160" + response + "&#160&#9835";
      let controlbar = document.getElementById("control");
      controlbar!.style.display = "initial";
      let convertBtn = document.getElementById("convert2Radio");
      convertBtn!.style.display = "initial";
      let playConvertBtn = document.getElementById("playRadioBtn");
      playConvertBtn!.style.display = "none";
    })
})

function ChangeControl() {
  if (CurrentState == MusicState.play) {
    document.getElementById("playBtn")!.innerHTML = "&#9834 Play";
    document.getElementById("pauseBtn")!.innerHTML = "Pause";
    document.getElementById("stopBtn")!.innerHTML = "Stop";
    document.getElementById("playRadioBtn")!.innerHTML = "Play Radio";
    document.getElementById("musicanimation")!.classList.add('on');
  } else if (CurrentState == MusicState.pause) {
    document.getElementById("playBtn")!.innerHTML = "Play";
    document.getElementById("pauseBtn")!.innerHTML = "&#9834 Pause";
    document.getElementById("stopBtn")!.innerHTML = "Stop";
    document.getElementById("playRadioBtn")!.innerHTML = "Play Radio";
    document.getElementById("musicanimation")!.classList.remove('on')
  } else if (CurrentState == MusicState.stop) {
    document.getElementById("playBtn")!.innerHTML = "Play";
    document.getElementById("pauseBtn")!.innerHTML = "Pause";
    document.getElementById("stopBtn")!.innerHTML = "&#9834 Stop";
    document.getElementById("playRadioBtn")!.innerHTML = "Play Radio";
  } else if (CurrentState == MusicState.playradio) {
    document.getElementById("playBtn")!.innerHTML = "Play";
    document.getElementById("pauseBtn")!.innerHTML = "Pause";
    document.getElementById("stopBtn")!.innerHTML = "Stop";
    document.getElementById("playRadioBtn")!.innerHTML = "&#9834 Play Radio";
  }
}

const onPlay = () => {
  invoke<boolean>('play_music2')
    .then((response) => {
      if (response) {
        CurrentState = MusicState.play;
        ChangeControl();
      }
    })
}

const onPause = () => {
  invoke<boolean>('pause_music')
    .then((response) => {
      if (response) {
        CurrentState = MusicState.pause;
        ChangeControl();
      }
    })
}

const onStop = () => {
  invoke<boolean>('stop_music')
    .then((response) => {
      if (response) {
        CurrentState = MusicState.stop;
        ChangeControl();
      }
    })
}

const onToggle = () => {

  if (CurrentState == MusicState.play) {
    invoke<boolean>('pause_music')
      .then((response) => {
        if (response) {
          CurrentState = MusicState.pause;
          ChangeControl();
        }
      });
  } else {
    invoke<boolean>('play_music2')
      .then((response) => {
        if (response) {
          CurrentState = MusicState.play;
          ChangeControl();
        }
      });
  }
}

const onPlayRadio = () => {

  invoke<boolean>('on_radio')
    .then((response) => {
      if (response) {
        CurrentState = MusicState.playradio;
        ChangeControl();
      }
    })
}

const onConvert2Radio = () => {

  let playConvertBtn = document.getElementById("playRadioBtn");
  playConvertBtn!.style.display = "none";
  let convertBtn = document.getElementById("convert2Radio");
  convertBtn!.style.display = "none";
  invoke<boolean>('convert2radio')
    .then((response) => {
      (document.getElementById("convert2Radio")! as HTMLButtonElement).disabled = false;
      if (response) {
        playConvertBtn!.style.display = "initial";
      }
    })
}


export default function Home() {
  return (
    <>
      <div className={styles.playmusic}>
        <div id="musicanimation" className={styles.musicanimation}>
          <span className={`${styles.bar} ${styles.bar1}`}></span>
          <span className={`${styles.bar} ${styles.bar2}`}></span>
          <span className={`${styles.bar} ${styles.bar3}`}></span>
          <span className={`${styles.bar} ${styles.bar4}`}></span>
        </div>
        <div className={styles.songname}>
          <a onClick={onToggle} id="toggle" >Drag & drop music to listen...</a>
        </div>
      </div>
      <div id="control" className={styles.control}>
        <div className={styles.musictoggle}>
          <a onClick={onPlay} id="playBtn" > Play</a>
        </div>
        <div className={styles.musictoggle}>
          <a onClick={onPause} id="pauseBtn"> Pause</a>
        </div>
        <div className={styles.musictoggle}>
          <a onClick={onStop} id="stopBtn" > Stop</a>
        </div>
        <div className={styles.musictoggle}>
          <a onClick={onConvert2Radio} id="convert2Radio" > Convert to Radio</a>
        </div>
        <div className={styles.musictoggle}>
          <a onClick={onPlayRadio} id="playRadioBtn" > Play Radio</a>
        </div>
      </div>

    </>
  )
}
