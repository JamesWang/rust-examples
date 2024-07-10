use std::fs::File;
use std::io::{BufReader, Result};
use log::info;
use simple_logger::SimpleLogger;

pub fn exp_with_mp3() {
    SimpleLogger::new().init().unwrap();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file = File::open("/home/jooly/Music/漂洋过海来看你.mp3").unwrap();
    let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(decoder);
    sink.sleep_until_end();
    /* loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    } */

}

fn main() -> Result<()> {
    exp_with_mp3();
    Ok(())
}