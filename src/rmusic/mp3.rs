use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;


pub struct Mp3Decoder<R> where R: Read + Seek {
    reader: rodio::Decoder<R>,
    current_frame_channel: usize,
    current_frame_sample_pos: usize,
    current_time: u64,
}
