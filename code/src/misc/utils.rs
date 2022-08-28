use std::{path::Path, fs};

use chrono::{prelude::*, format::{DelayedFormat, StrftimeItems}};

use crate::settings::HttpHeader;

pub fn format_date_time(unix_timestamp: u64, format: &str) -> DelayedFormat<StrftimeItems>
{
    let naive_date_time = NaiveDateTime::from_timestamp(unix_timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);
    
    return datetime.format(format);
}

pub fn read_file_bytes(file_path: &str) -> Vec<u8>
{
    if Path::new(file_path).exists()
    {
        match fs::read(file_path)
        {
            Ok(v) => v,
            Err(_) => Vec::new()
        }
    }
    else {
        Vec::new()
    }
}

pub fn config_http_headers_to_httparse_headers(
    httparse_headers: &mut Vec<httparse::Header>,
    config_http_headers: &'static Vec<HttpHeader>
)
{
    for (index, http_header) in config_http_headers.iter().enumerate() {
        httparse_headers[index].name = http_header.name.as_str();
        httparse_headers[index].value = http_header.value.as_bytes();
    }
}