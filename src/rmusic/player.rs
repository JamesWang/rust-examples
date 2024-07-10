use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crossbeam::sync::SegQueue;
use log::info;
use rodio::Source;


use self::Action::*;


#[derive(Debug)]
enum Action {
    Load(PathBuf), Pause, Stop, Resume,
}

#[derive(Clone)]
struct EventLoop {
    condition_variable: Arc<(Mutex<bool>, Condvar)>,
    queue: Arc<SegQueue<Action>>,
    playing: Arc<Mutex<bool>>,
}

impl EventLoop {
    fn new() -> Self {
        EventLoop {
            condition_variable: Arc::new((Mutex::new(false), Condvar::new())),
            queue: Arc::new(SegQueue::new()),
            playing: Arc::new(Mutex::new(false)),
        }
    }
}

pub struct State {
    pub current_time: u64,
    pub durations: HashMap<String, u64>,
    pub stopped: bool,
}

pub struct Player {
    app_state: Arc<Mutex<State>>,
    event_loop: EventLoop,
    paused: Cell<bool>,
}

impl Player {
    pub (crate) fn new(app_state: Arc<Mutex<State>>) -> Self {
        let event_loop = EventLoop::new();
        {
            let app_state = app_state.clone();
            let event_loop = event_loop.clone();
            thread::spawn(move||{            
                let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&handle).unwrap();
                let mut decoder: rodio::Decoder<BufReader<File>>;
                loop {
                    if let Some(action) = event_loop.queue.try_pop() {
                        info!("receive action={:?}", action);
                        match action {
                            Load(path) => {
                                info!("open music file...");
                                let mut file = File::open(&path).unwrap();
                                file.seek(std::io::SeekFrom::Start(0)).unwrap();                                
                                decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();                                                                
                                let total_duration = decoder.total_duration().unwrap().as_secs();
                                sink.append(decoder);                                

                                app_state.lock().unwrap().durations.insert(path.to_str().unwrap().to_string(), total_duration);
                            },
                            Pause => {
                                sink.pause();                                                            
                            },
                            Resume => {
                                sink.play();
                            },
                            Stop => {
                                sink.stop();
                            },
                        }
                    } else if *event_loop.playing.lock().unwrap() {                    
                        //
                    }
                    //let cur_pos = sink.get_pos().as_secs() /    
                    app_state.lock().unwrap().current_time = sink.get_pos().as_secs();
                }

            });
        }
        Player {
            app_state, 
            event_loop,
            paused: Cell::new(false),
            //sink,
        }
    }
    
    pub fn load(&self, path: &String) {
       self.event_loop.queue.push(Load(PathBuf::from(path)));
    }

    pub fn is_paused(&self) -> bool {
        self.paused.get()
    }

    pub fn pause(&self) {
        self.paused.set(true);
        self.app_state.lock().unwrap().stopped = true;
        self.set_playing(false);
        self.emit(Pause);
    }

    pub fn resume(&self) {
        self.paused.set(true);        
        self.set_playing(false);
        self.emit(Resume);
    }
    fn set_playing(&self, playing: bool) {
        *self.event_loop.playing.lock().unwrap() = playing;
        let (ref lock, ref condition_variable) = *self.event_loop.condition_variable;
        let mut started = lock.lock().unwrap();
        *started = playing;
        if playing {
            condition_variable.notify_one();
        }
    }

    pub fn stop(&self) {
        self.paused.set(false);
        self.app_state.lock().unwrap().stopped = true;
        self.emit(Stop);
        self.set_playing(false);
    }
    fn emit(&self, action: Action) {
        self.event_loop.queue.push(action);
    }
}