use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;



fn main() {

	// TODO handle SIGINT, SIGTERM, etc.

	println!("Starting go server on port 80");
	let listener = TcpListener::bind("0.0.0.0:80");

	match listener {
		Ok(result) => {
			// TODO match here too
			listen_incoming_connections(result);
		},
		Err(err) => println!("{}", err)
	}

}

/*
enum HttpMethod {
	Options,
	Get,
	Post,
	Put,
	Delete
}

enum HttpVersion {
	Undefined,
	11,
	2
}
*/

fn listen_incoming_connections(listener:std::net::TcpListener) -> Result<(), std::io::Error> {

	for stream in listener.incoming() {
		handle_connection(stream?)?;
	}

	Ok(())

}

fn handle_connection(mut stream: TcpStream) -> Result<usize, std::io::Error> {

	let mut buffer = [0; 1024];
	let buffer_result = stream.read(&mut buffer)?; // TODO implement timeouts

	// formato das requisições
	// https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html

	// methods and root uri
	let options = b"OPTIONS / ";
	let get = b"GET / ";

	// let http_method = HttpMethod::Get;
	// let uri = "";
	// let http_version = HttpVersion::11;

	if buffer.starts_with(options) {
		handle_options(buffer, stream)?;
	} else if buffer.starts_with(get) {
		handle_get(buffer, stream)?;
	} else {

		println!("TODO 404")
		/*(
			"HTTP/1.1 404 NOT FOUND",
			"content-type: text/html".to_string(),
			"404.html"
		)*/

	};

	Ok(buffer_result)

}

// formato das respostas
// https://www.w3.org/Protocols/rfc2616/rfc2616-sec6.html

fn handle_options(buffer:[u8; 1024], mut stream: TcpStream) -> Result<(), std::io::Error> {

	let (status_line, headers) = (
		"HTTP/1.1 200 OK",
		concat!(
			"access-control-allow-origin: *", // posso usar *, mas não se requerir autenticação com bearer:
			"access-control-allow-methods: options, get, post, put, delete",
			"access-control-allow-headers: content-type", // posso usar *
		)
		.to_string()
	);

	let response = format!("{}\r\n{}", status_line, headers);

	// TODO implement timeouts
	stream.write(response.as_bytes())?;
	stream.flush()?;

	Ok(())

}

fn handle_get(buffer:[u8; 1024], mut stream: TcpStream) -> Result<(), std::io::Error> {

	let (status_line, mut headers, filename) = (
		"HTTP/1.1 200 OK",
		"content-type: application/json".to_string(),
		"hello.json"
	);

	let body = fs::read_to_string(filename)?;

	headers = [
		headers,
		"\r\ncontent-length: ".to_string(),
		body.len().to_string()
	]
	.concat();

	let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, body);

	// TODO implement timeouts
	stream.write(response.as_bytes())?;
	stream.flush()?;

	Ok(())

}