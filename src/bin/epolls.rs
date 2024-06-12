#[path ="../epolls/ffi/mod.rs"]
mod ffi ;

#[path ="../epolls/poll/mod.rs"]
mod poll ;

use std::{io::{self, Read, Result, Write}, net::TcpStream};
use ffi::mffi::{Event,EPOLLET, EPOLLIN};
use poll::mpoll;

fn get_req(path: &str)-> Vec<u8> {
    format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").into()
}

fn main() {

    let mut poll = mpoll::Poll::new()?;

    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";
    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, EPOLLIN | EPOLLET)?;
        streams.push(stream);    
    }
}