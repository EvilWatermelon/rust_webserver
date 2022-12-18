use std::{
	fs,
	io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	listener.set_nonblocking(true).expect("Cannot set non-blocking");

	for stream in listener.incoming() {
		match stream {
			Ok(s) => {
				handle_connection(s);
			}
			Err(ref e) if(e.kind() == io::ErrorKind::WouldBlock) => {
				continue;
			}
			Err(e) => panic!("Encountered IO error: {}", e),
		}	
		
	}
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&mut stream);
	let request_line = buf_reader.lines().next().unwrap().unwrap();

	let (status_line, filename) = if(request_line == "GET / HTTP/1.1") {
		("HTTP/1.1 200 OK", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};
	
	let contents = fs::read_to_string(filename).unwrap();
	let length = contents.len();

	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents
	);

	stream.write_all(response.as_bytes()).unwrap();
}
