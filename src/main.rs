#![allow(unused)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use baller::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    let file_path = match path {
        "/" => "static/index.html",
        "/script.js" => "static/script.js",
        _ => "static/404.html",
    };

    let contents = fs::read_to_string(file_path).unwrap_or_else(|_| "404 Not Found".to_string());

    let content_type = if path.ends_with(".js") {
        "text/javascript"
    } else {
        "text/html"
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        contents.len(),
        contents,
    );

    stream.write_all(response.as_bytes()).unwrap();
}
