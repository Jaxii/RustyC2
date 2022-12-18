use httparse::{self, Error, Header, Response, Status};
use lazy_static::lazy_static;
use regex;
use regex::bytes::Regex;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time;

use crate::database;
use crate::misc::utils;
use crate::models::{
    ImplantConnectionType, ImplantTask, ImplantTaskStatus, ListenerSignal, ListenerStatus,
};
use crate::settings;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new();
}

fn handle_connection(mut stream: TcpStream, listener_id: u16) {
    let mut http_request_bytes: Vec<u8> = Vec::new();
    let mut is_implant: bool = false;
    let mut implant_connection_type: ImplantConnectionType = ImplantConnectionType::Pull;
    let mut implant_auth_cookie: String = String::new();
    let http_method: String;
    let http_path: String;

    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(v) => {
                if v == 0 {
                    break;
                } else {
                    http_request_bytes.extend_from_slice(&buffer);
                    
                    if v < buffer.len()
                    {
                        break;
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    let regex_double_crlf = Regex::new(r"\r\n\r\n").unwrap();
    let mut double_crlf_offset: usize = http_request_bytes.len() - 1;
    match regex_double_crlf.find(&http_request_bytes) {
        Some(m) => double_crlf_offset = m.start(),
        None => {}
    }

    if double_crlf_offset > http_request_bytes.len() {
        println!("[!] TODO: Offset of the double CRLF is greater than the size of the buffer. Investigate!");
        return;
    }

    let re: Regex = Regex::new(r"\r\n").unwrap();
    let num_http_headers: usize = re
        .find_iter(&http_request_bytes[0..double_crlf_offset])
        .count();
    let mut http_headers: Vec<httparse::Header> = vec![
        Header {
            name: "",
            value: &[]
        };
        num_http_headers
    ];

    let mut req = httparse::Request::new(&mut http_headers);
    let res: Result<Status<usize>, Error> = req.parse(&http_request_bytes);

    if res.is_err() {
        println!("[!] Error parsing the HTTP request");
        return;
    }

    if req.method.is_some() && req.path.is_some() {
        http_method = req.method.unwrap().to_string();
        http_path = req.path.unwrap().to_string();

        // now that we know the method and the path, we must check
        // the auth. cookie to determine if it's an implant

        let regex_string = &CONFIG.listener.http.auth_cookie_regex;
        match Regex::new(regex_string.as_str()) {
            Ok(re) => match re.captures(&http_request_bytes) {
                Some(caps) => match caps.get(1) {
                    Some(capture_match) => {
                        let cookie: Vec<u8> =
                            http_request_bytes[capture_match.start()..capture_match.end()].to_vec();

                        match String::from_utf8(cookie) {
                            Ok(cookie_str) => {
                                if http_method == CONFIG.listener.http.pull_method
                                    && http_path == CONFIG.listener.http.pull_endpoint
                                {
                                    implant_auth_cookie = cookie_str;
                                    implant_connection_type = ImplantConnectionType::Pull;
                                    is_implant = true;
                                } else if http_method == CONFIG.listener.http.push_method
                                    && http_path == CONFIG.listener.http.push_endpoint
                                {
                                    implant_auth_cookie = cookie_str;
                                    implant_connection_type = ImplantConnectionType::Push;
                                    is_implant = true;
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    None => {}
                },
                None => {}
            },
            Err(_) => {
                println!("[!] The regex you specified in the configuration (for the HTTP listener) isn't valid");
                return;
            }
        }
    } else {
        // couldn't parse the HTTP request, there's something wrong going on
        return;
    }

    let http_response_bytes: Vec<u8> = if is_implant {
        let http_request_body_offset: Status<usize> = res.unwrap();
        let http_request_body_bytes: &[u8] =
            &http_request_bytes[http_request_body_offset.unwrap()..];

        prepare_http_response_implant(
            listener_id,
            implant_connection_type,
            implant_auth_cookie,
            http_request_body_bytes,
        )
    } else {
        prepare_http_response(http_method, http_path)
    };

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    match stream.write_all(&http_response_bytes) {
        Ok(_) => {
            if stream.flush().is_err() {
                println!("[!] Error flushing the stream");
            }
        }
        Err(_) => {
            println!("[!] Error sending the HTTP response bytes to the client");
        }
    }
}

pub fn start_listener(listener_id: u16, rx: Receiver<ListenerSignal>) {
    let address: String;
    let port: u16;

    address = crate::database::get_listener_address(listener_id);
    port = crate::database::get_listener_port(listener_id);

    let bind_address: String = String::from(&format!("{}:{}", address, port));

    let bind_result: Result<TcpListener, io::Error> = TcpListener::bind(bind_address);
    if bind_result.is_err() {
        println!("\n[!] Couldn't bind the listener to the specified address");
        println!("[!] Is it already in use?");
        return;
    } else {
        if crate::database::set_listener_status(listener_id, ListenerStatus::Active) {
            println!("\n[+] Set listener as active");
        } else {
            println!("\n[!] Error while setting the listener as active");
            return;
        }

        let listener: TcpListener = bind_result.unwrap();
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    // handle the incoming connection
                    handle_connection(s, listener_id);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(time::Duration::from_secs(1));

                    let msg_result = rx.try_recv();
                    match msg_result {
                        Ok(b) => match b {
                            ListenerSignal::StopListener => return,
                        },
                        Err(TryRecvError::Disconnected) => return,
                        Err(TryRecvError::Empty) => continue,
                    }
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }
}

fn prepare_http_response(http_method: String, http_path: String) -> Vec<u8> {
    let default_page_path: String = (&CONFIG.listener.http.default_page_path).to_string();
    let error_page_path: String = (&CONFIG.listener.http.default_error_page_path).to_string();

    let mut http_response: Response = httparse::Response::new(&mut []);
    let _http_response_headers: Vec<Header> = Vec::new();

    if http_method == "GET" && http_path == "/" {
        http_response.code = Some(CONFIG.listener.http.responses.default_success.status_code);
        http_response.version = Some(CONFIG.listener.http.responses.default_success.http_version);
        http_response.reason = Some(
            &CONFIG
                .listener
                .http
                .responses
                .default_success
                .status_code_reason,
        );

        let mut new_http_headers: Vec<httparse::Header> =
            vec![
                Header {
                    name: "",
                    value: &[]
                };
                CONFIG.listener.http.responses.default_success.headers.len()
            ];
        utils::config_http_headers_to_httparse_headers(
            new_http_headers.as_mut(),
            &CONFIG.listener.http.responses.default_success.headers,
        );
        http_response.headers = new_http_headers.as_mut();

        write_http_response_bytes(http_response, utils::read_file_bytes(&default_page_path))
    } else {
        http_response.code = Some(CONFIG.listener.http.responses.default_error.status_code);
        http_response.version = Some(CONFIG.listener.http.responses.default_error.http_version);
        http_response.reason = Some(
            &CONFIG
                .listener
                .http
                .responses
                .default_error
                .status_code_reason,
        );

        let mut new_http_headers: Vec<httparse::Header> =
            vec![
                Header {
                    name: "",
                    value: &[]
                };
                CONFIG.listener.http.responses.default_error.headers.len()
            ];
        utils::config_http_headers_to_httparse_headers(
            new_http_headers.as_mut(),
            &CONFIG.listener.http.responses.default_error.headers,
        );
        http_response.headers = new_http_headers.as_mut();

        write_http_response_bytes(http_response, utils::read_file_bytes(&error_page_path))
    }
}

fn prepare_http_response_task_command(task_command: String) -> Vec<u8> {
    let http_response_bytes: Vec<u8> = task_command.as_bytes().to_vec();
    return http_response_bytes;
}

fn prepare_http_response_implant(
    listener_id: u16,
    implant_connection_type: ImplantConnectionType,
    implant_auth_cookie: String,
    http_request_body_bytes: &[u8],
) -> Vec<u8> {
    let mut http_response: Response = httparse::Response::new(&mut []);
    let _http_response_headers: Vec<Header> = Vec::new();
    let mut http_response_body: Vec<u8> = Vec::new();

    match implant_connection_type {
        ImplantConnectionType::Pull => {
            if database::check_if_implant_exists(None, Some(&implant_auth_cookie)).is_none() {
                println!(
                    "\n[+] Adding to database implant with hash: {}",
                    &implant_auth_cookie
                );

                if database::add_implant(listener_id, &implant_auth_cookie) {
                    println!("[+] Implant successfully added to the database");

                    http_response.code = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_success
                            .status_code,
                    );
                    http_response.version = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_success
                            .http_version,
                    );
                    http_response.reason = Some(
                        &CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_success
                            .status_code_reason,
                    );

                    let mut new_http_headers: Vec<httparse::Header> = vec![
                        Header {
                            name: "",
                            value: &[]
                        };
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_success
                            .headers
                            .len()
                    ];
                    utils::config_http_headers_to_httparse_headers(
                        new_http_headers.as_mut(),
                        &CONFIG.listener.http.responses.implant_pull_success.headers,
                    );
                    http_response.headers = new_http_headers.as_mut();
                    return write_http_response_bytes(http_response, http_response_body);
                } else {
                    println!("[!] Failed to add the implant to the database");
                }
            } else {
                if !database::update_implant_timestamp(&implant_auth_cookie) {
                    println!("[!] Couldn't update the timestamp of the implant with this cookie");
                    println!("\t{}", &implant_auth_cookie);
                }

                let mut include_statuses: Vec<String> = Vec::new();
                include_statuses.push(ImplantTaskStatus::Issued.to_string());

                let implant_tasks: Vec<ImplantTask> = database::get_implant_tasks(
                    "CookieHash",
                    &implant_auth_cookie,
                    include_statuses,
                );

                if implant_tasks.len() == 0 {
                    http_response.code = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_failure
                            .status_code,
                    );
                    http_response.version = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_failure
                            .http_version,
                    );
                    http_response.reason = Some(
                        &CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_failure
                            .status_code_reason,
                    );

                    let mut new_http_headers: Vec<httparse::Header> = vec![
                        Header {
                            name: "",
                            value: &[]
                        };
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_pull_failure
                            .headers
                            .len()
                    ];
                    utils::config_http_headers_to_httparse_headers(
                        new_http_headers.as_mut(),
                        &CONFIG.listener.http.responses.implant_pull_failure.headers,
                    );
                    http_response.headers = new_http_headers.as_mut();

                    return write_http_response_bytes(http_response, http_response_body);
                } else {
                    for task in implant_tasks {
                        if !database::update_implant_task_status(
                            task.id,
                            ImplantTaskStatus::Pending,
                        ) {
                            println!("[!] Couldn't update the status of the task");

                            // since we failed to update the status of the task, we
                            // can't send it to the implant
                            // so just assume that there are no tasks
                            http_response.code = Some(
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_failure
                                    .status_code,
                            );
                            http_response.version = Some(
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_failure
                                    .http_version,
                            );
                            http_response.reason = Some(
                                &CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_failure
                                    .status_code_reason,
                            );

                            let mut new_http_headers: Vec<httparse::Header> = vec![
                                Header {
                                    name: "",
                                    value: &[]
                                };
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_failure
                                    .headers
                                    .len()
                            ];
                            utils::config_http_headers_to_httparse_headers(
                                new_http_headers.as_mut(),
                                &CONFIG.listener.http.responses.implant_pull_failure.headers,
                            );
                            http_response.headers = new_http_headers.as_mut();

                            return write_http_response_bytes(http_response, http_response_body);
                        } else {
                            http_response.code = Some(
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_success
                                    .status_code,
                            );
                            http_response.version = Some(
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_success
                                    .http_version,
                            );
                            http_response.reason = Some(
                                &CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_success
                                    .status_code_reason,
                            );
                            http_response_body = prepare_http_response_task_command(task.command);

                            let mut new_http_headers: Vec<httparse::Header> = vec![
                                Header {
                                    name: "",
                                    value: &[]
                                };
                                CONFIG
                                    .listener
                                    .http
                                    .responses
                                    .implant_pull_success
                                    .headers
                                    .len()
                            ];
                            utils::config_http_headers_to_httparse_headers(
                                new_http_headers.as_mut(),
                                &CONFIG.listener.http.responses.implant_pull_success.headers,
                            );
                            http_response.headers = new_http_headers.as_mut();

                            return write_http_response_bytes(http_response, http_response_body);
                        }
                    }
                }
            }

            // return the http response bytes
            return write_http_response_bytes(http_response, http_response_body);
        }
        ImplantConnectionType::Push => {
            let implant_id = database::check_if_implant_exists(None, Some(&implant_auth_cookie));

            if implant_id.is_none() {
                http_response.code = Some(
                    CONFIG
                        .listener
                        .http
                        .responses
                        .implant_push_failure
                        .status_code,
                );
                http_response.version = Some(
                    CONFIG
                        .listener
                        .http
                        .responses
                        .implant_push_failure
                        .http_version,
                );
                http_response.reason = Some(
                    &CONFIG
                        .listener
                        .http
                        .responses
                        .implant_push_failure
                        .status_code_reason,
                );

                let mut new_http_headers: Vec<httparse::Header> = vec![
                    Header {
                        name: "",
                        value: &[]
                    };
                    CONFIG
                        .listener
                        .http
                        .responses
                        .implant_push_failure
                        .headers
                        .len()
                ];
                utils::config_http_headers_to_httparse_headers(
                    new_http_headers.as_mut(),
                    &CONFIG.listener.http.responses.implant_push_failure.headers,
                );
                http_response.headers = new_http_headers.as_mut();

                return write_http_response_bytes(http_response, http_response_body);
            }

            if http_request_body_bytes.len() > 0 {
                if database::update_implant_task_output(
                    implant_id.unwrap(),
                    http_request_body_bytes,
                ) {
                    http_response.code = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_success
                            .status_code,
                    );
                    http_response.version = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_success
                            .http_version,
                    );
                    http_response.reason = Some(
                        &CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_success
                            .status_code_reason,
                    );

                    let mut new_http_headers: Vec<httparse::Header> = vec![
                        Header {
                            name: "",
                            value: &[]
                        };
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_success
                            .headers
                            .len()
                    ];
                    utils::config_http_headers_to_httparse_headers(
                        new_http_headers.as_mut(),
                        &CONFIG.listener.http.responses.implant_push_success.headers,
                    );
                    http_response.headers = new_http_headers.as_mut();

                    return write_http_response_bytes(http_response, http_response_body);
                } else {
                    http_response.code = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_failure
                            .status_code,
                    );
                    http_response.version = Some(
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_failure
                            .http_version,
                    );
                    http_response.reason = Some(
                        &CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_failure
                            .status_code_reason,
                    );

                    let mut new_http_headers: Vec<httparse::Header> = vec![
                        Header {
                            name: "",
                            value: &[]
                        };
                        CONFIG
                            .listener
                            .http
                            .responses
                            .implant_push_failure
                            .headers
                            .len()
                    ];
                    utils::config_http_headers_to_httparse_headers(
                        new_http_headers.as_mut(),
                        &CONFIG.listener.http.responses.implant_push_failure.headers,
                    );
                    http_response.headers = new_http_headers.as_mut();

                    return write_http_response_bytes(http_response, http_response_body);
                }
            }

            return write_http_response_bytes(http_response, http_response_body);
        }
    }
}

// https://github.com/vi/http-bytes/blob/master/src/lib.rs
pub fn write_http_response_bytes(
    http_response: httparse::Response,
    http_response_body_bytes: Vec<u8>,
) -> Vec<u8> {
    let code = http_response.code.unwrap();
    let reason = http_response.reason.unwrap();
    let headers = http_response.headers;
    let mut output_vector: Vec<u8> = Vec::new();

    output_vector.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", code, reason).as_bytes());

    for header in headers {
        output_vector.extend_from_slice(format!("{}: ", header.name).as_bytes());
        output_vector.extend_from_slice(header.value);
        output_vector.extend_from_slice(b"\r\n");
    }

    output_vector.extend_from_slice(b"\r\n");
    output_vector.extend_from_slice(http_response_body_bytes.as_slice());

    return output_vector;
}
