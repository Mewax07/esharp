use std::{net::TcpListener, sync::Arc, thread};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
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
        let listener =
            TcpListener::bind(&address).expect(&format!("Connection to {} failed", address));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream = Arc::new(stream);
                    thread::spawn(move || {
                        let _stream = Arc::try_unwrap(stream).unwrap();
                        // handle_request();
                    });
                }
                Err(e) => {
                    eprintln!("Connection error: {e}")
                }
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
