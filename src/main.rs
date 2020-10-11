use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;



fn main() {

	let listener = TcpListener::bind("0.0.0.0:80").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
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

fn handle_connection(mut stream: TcpStream) {

	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	// formato das requisições
	// https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html

	// methods and root uri
	let options = b"OPTIONS / ";
	let get = b"GET / ";

	// let http_method = HttpMethod::Get;
	// let uri = "";
	// let http_version = HttpVersion::11;

	if buffer.starts_with(options) {
		handle_options(buffer, stream);
	} else if buffer.starts_with(get) {
		handle_get(buffer, stream);
	} else {

		print!("TODO 404")
		/*(
			"HTTP/1.1 404 NOT FOUND",
			"content-type: text/html".to_string(),
			"404.html"
		)*/

	};

}

// formato das respostas
// https://www.w3.org/Protocols/rfc2616/rfc2616-sec6.html

fn handle_options(buffer:[u8; 1024], mut stream: TcpStream) {

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

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();

}

fn handle_get(buffer:[u8; 1024], mut stream: TcpStream) {

	// TODO

	let (status_line, mut headers, filename) = (
		"HTTP/1.1 200 OK",
		"content-type: application/json".to_string(),
		"hello.json"
	);

	let body = fs::read_to_string(filename).unwrap();

	headers = [
		headers,
		"\r\ncontent-length: ".to_string(),
		body.len().to_string()
	]
	.concat();

	let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, body);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();

}