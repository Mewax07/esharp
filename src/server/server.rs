use std::{
    io::{Read, Write}, net::{TcpListener, TcpStream}, thread,
};

use crate::server::{Request, Response, handle_request};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn run(&self) {
        let address = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(&address).expect("Failed to bind server");

        println!("ESharp running on http://{}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        handle_connection(stream);
                    });
                }

                Err(error) => {
                    eprintln!("Connection error: {}", error);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 16 * 1024];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => return,
    };

    let raw = String::from_utf8_lossy(&buffer[..bytes_read]);

    let response = match Request::parse(&raw) {
        Some(request) => handle_request(request),

        None => Response::bad_request("Invalid HTTP request"),
    };

    let bytes = response.to_bytes();

    let _ = stream.write_all(&bytes);
    let _ = stream.flush();
}
