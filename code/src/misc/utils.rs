use std::{
    fs,
    path::Path,
    time::{Duration, SystemTime, SystemTimeError},
};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};

use crate::{
    database,
    models::{GenericImplant, HTTPListener},
    settings::HttpHeader,
};

pub fn format_date_time(unix_timestamp: u64, format: &str) -> DelayedFormat<StrftimeItems> {
    let naive_date_time = NaiveDateTime::from_timestamp(unix_timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);

    return datetime.format(format);
}

pub fn read_file_bytes(file_path: &str) -> Vec<u8> {
    if Path::new(file_path).exists() {
        match fs::read(file_path) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

pub fn config_http_headers_to_httparse_headers(
    httparse_headers: &mut Vec<httparse::Header>,
    config_http_headers: &'static Vec<HttpHeader>,
) {
    for (index, http_header) in config_http_headers.iter().enumerate() {
        httparse_headers[index].name = http_header.name.as_str();
        httparse_headers[index].value = http_header.value.as_bytes();
    }
}

pub fn list_listeners() {
    let listeners: Vec<HTTPListener> = database::get_http_listeners();

    if listeners.is_empty() {
        println!("[+] No listeners found");
    } else {
        println!("+----+------------+-----------------+-------+");
        println!("| ID |   STATUS   |     ADDRESS     |  PORT |");
        println!("+----+------------+-----------------+-------+");

        for listener in listeners.iter() {
            println!(
                "| {0:^2} | {1:^10} | {2:^15} | {3:^5} |",
                listener.id,
                listener.status.to_string(),
                listener.address,
                listener.port
            );
        }

        println!("+----+------------+-----------------+-------+");
    }
}

pub fn list_implants() {
    let implants: Vec<GenericImplant> = database::get_implants();

    if implants.is_empty() {
        println!("[+] No implants found");
    } else {
        println!("+----+------------+-----------------+");
        println!("| ID |  Listener  |    Last Seen    |");
        println!("+----+------------+-----------------+");

        let time_elapsed_now: Result<Duration, SystemTimeError> =
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        let time_now_seconds: u64 = time_elapsed_now.as_ref().unwrap().as_secs();

        for implant in implants {
            let last_seen_string: String = if !time_elapsed_now.is_err() {
                let time_diff_seconds = time_now_seconds - implant.last_seen;

                if time_diff_seconds <= 59 {
                    format!("{}s ago", time_diff_seconds)
                } else if time_diff_seconds <= 3599 {
                    format!(
                        "{}m {}s ago",
                        time_diff_seconds / 60,
                        time_diff_seconds % 60
                    )
                } else {
                    format!(
                        "{}h {}m ago",
                        time_diff_seconds / 3600,
                        (time_diff_seconds % 3600) / 60
                    )
                }
            } else {
                format!("{}", implant.last_seen)
            };

            println!(
                "| {0:^2} | {1:^10} | {2:^15} |",
                implant.id, implant.listener_id, last_seen_string,
            );
        }

        println!("+----+------------+-----------------+");
    }
}
