use std::{collections::HashMap, io::{Read, Write}, net::TcpStream};

pub fn handle_request(mut stream: TcpStream) {
	let mut buffer = [0; 1024 * 4];
	let bytes_read = stream.read(&mut buffer).unwrap_or(0);
	if bytes_read == 0 { return; }

	let request = String::from_utf8_lossy(&buffer[..bytes_read]);
	let path = if let Some(first_line) = request.lines().next() {
		first_line.split_whitespace().nth(1).unwrap_or("/")
	} else { "/" };

	let (path, query_params) = if let Some(query_start) = path.find('?') {
		let (base_path, query) = path.split_at(query_start);
		let params = HashMap::new();
		(base_path, params)
	} else { (path, HashMap::new()) };

	let body = if request.contains("\r\n\r\n") {
		let body_start = request.find("\r\n\r\n").unwrap() + 4;
		&request[body_start..]
	} else { "" };

	// add handle with a response match for content (api)
	stream.flush().unwrap();
}
