use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crossbeam::sync::SegQueue;
use pulse_simple::Playback;
use super::mp3::Mp3Decoder;
use self::Action::*;

const BUFFER_SIZE: usize = 1000;
const DEFAULT_RATE: u32 = 44100;

enum Action {
    Load(PathBuf), Stop,
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
                let mut buffer = [[0; 2]; BUFFER_SIZE];
                let mut playback = Playback::new("MP3", "MP3 Playback", None, DEFAULT_RATE);
                let mut source = None;
                loop {
                    if let Some(action) = event_loop.queue.try_pop() {
                        match action {
                            Load(path) => {
                                let file = File::open(path).unwrap();
                                source = Some(Mp3Decoder::new(BufReader::new(file)).unwrap());
                                let rate = source.as_ref().map(|source|source.sample_rate()).unwrap_or(DEFAULT_RATE);
                                playback = Playback::new("MP3","MP3 Playback", None, rate);
                                app_state.lock().unwrap().stopped = false;
                            },
                            Stop => {},
                        }
                    } else if *event_loop.playing.lock().unwrap() {
                        let mut written = false;
                        if let Some(ref mut source) = source {
                            let size = Self::iter_to_buffer(source, &mut buffer);
                            if size > 0 {
                                playback.write(&buffer[..size]);
                                written = true;
                            }
                        }
                        if !written {
                            app_state.lock().unwrap().stopped = true;
                            *event_loop.playing.lock().unwrap() = false;
                            source = None;
                        }
                    }
                }

            });
        }
        Player {
            app_state, 
            event_loop,
            paused: Cell::new(false),
        }
    }

    fn iter_to_buffer<I: Iterator<Item=i16>>(iter: &mut I, buffer: &mut [[i16;2]; BUFFER_SIZE]) -> usize {
        let mut iter = iter.take(BUFFER_SIZE);
        let mut index = 0;
        while let Some(sample1) = iter.next(){
            if let Some(sample2) = iter.next() {
                buffer[index][0] = sample1;
                buffer[index][1] = sample2;
            }
            index +=1;
        }
        index
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
    }

    pub fn resume(&self) {
        self.paused.set(true);
        self.app_state.lock().unwrap().stopped = true;
        self.set_playing(false);
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
    pub fn compute_duration<P: AsRef<Path>>(path: P) -> Option<Duration> {
        let file = File::open(path).unwrap();
        Mp3Decoder::compute_duration(BufReader::new(file))
    }
}