use lazy_static::lazy_static;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::settings;

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };
    let contents = filename;

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn create(address: String, port: u16)
{
    let bind_address = String::from(&format!(
        "{}:{}",
        address, port
    ));

    let listener = TcpListener::bind(bind_address).unwrap();

    for stream in listener.incoming()
    {
        match stream {
            Ok(s) =>
            {
                // do something with the TcpStream
                handle_connection(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock =>
            {
                break;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}
