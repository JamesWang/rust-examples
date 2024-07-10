#[path ="../epolls/mod.rs"]
mod epolls;

#[path ="../epolls/ffi/mod.rs"]
mod ffi ;

#[path ="../epolls/poll/mod.rs"]
mod poll ;

use std::{io::{self, Read, Result, Write}, net::TcpStream};
use epolls::poll::mpoll::{self, Events};
use epolls::ffi::mffi::{Event,EPOLLET, EPOLLIN};


fn get_req(path: &str)-> String {
    format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").into()
}

fn main() -> Result<()> {

    let poll = mpoll::Poll::new()?;

    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";
    for i in 0..n_events {
        println!("{}", i);
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, EPOLLIN | EPOLLET)?;
        streams.push(stream);
    }
    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events: Events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("Timeout (Or Spurious Event Notification)");
            continue;
        }
        handled_events += handle_events(&events, &mut streams)?;
    }
    println!("FINISHED");
    Ok(())
}

fn handle_events(events: &Vec<Event>, streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];
        loop {
            match streams[index].read(&mut data) {
                Ok(0) => {
                    handled_events +=1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n--------\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}