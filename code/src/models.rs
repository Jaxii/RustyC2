use std::any::Any;
use std::net::{IpAddr, AddrParseError};
use std::fmt;
use std::num::ParseIntError;
use std::str::{FromStr, Chars};

use rusqlite::Row;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef, FromSqlError};

use crate::database;
pub struct GenericListener
{
    pub id: u16,
    pub protocol: ListenerProtocol,
    pub status: ListenerStatus,
    pub data: Box<dyn Any>
}

#[derive(Debug, PartialEq)]
pub struct HTTPListener
{
    pub address: IpAddr,
    pub port: u16,
    pub host: String
}

pub struct GenericImplant
{
    pub id: u16,
    pub listener_id: u16,
    pub last_seen: u64,
    pub data: Box<dyn Any>
}

pub struct ImplantTask
{
    pub id: u64,
    pub implant_id: u16,
    pub command: String,
    pub datetime: u64,
    pub status: ImplantTaskStatus,
    pub output: Vec<u8>
}

pub enum ListenerProtocol
{
    TCP,
    UDP,
    HTTP,
    ICMP,
    DNS
}

pub enum ListenerStatus
{
    Created,
    Active,
    Suspended
}

pub enum ListenerSignal
{
    StopListener
}

pub enum ImplantTaskStatus
{
    Issued,
    Pending,
    Completed
}

pub enum ImplantConnectionType
{
    Pull,
    Push
}

pub trait ManageSettings
{
    fn show_settings(&self);
    fn set_option(&mut self, option: &str, value: &str) -> bool;
}

impl fmt::Display for ListenerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ListenerStatus::Created => "Created".fmt(f),
           ListenerStatus::Active => "Active".fmt(f),
           ListenerStatus::Suspended => "Suspended".fmt(f),
       }
    }
}

impl fmt::Display for ListenerProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ListenerProtocol::TCP => "TCP".fmt(f),
            ListenerProtocol::UDP => "UDP".fmt(f),
            ListenerProtocol::HTTP => "HTTP".fmt(f),
            ListenerProtocol::ICMP => "ICMP".fmt(f),
            ListenerProtocol::DNS => "DNS".fmt(f),
       }
    }
}

impl fmt::Display for ImplantTaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ImplantTaskStatus::Issued => "Issued".fmt(f),
            ImplantTaskStatus::Pending => "Pending".fmt(f),
            ImplantTaskStatus::Completed => "Completed".fmt(f)
       }
    }
}

impl FromStr for ListenerStatus
{
    type Err = ();

    fn from_str(input: &str) -> Result<ListenerStatus, Self::Err> {
        match input {
            "Created"  => Ok(ListenerStatus::Created),
            "Active"  => Ok(ListenerStatus::Active),
            "Suspended"  => Ok(ListenerStatus::Suspended),
            _      => Err(()),
        }
    }
}

impl FromStr for ListenerProtocol
{
    type Err = ();

    fn from_str(input: &str) -> Result<ListenerProtocol, Self::Err> {
        match input {
            "TCP"  => Ok(ListenerProtocol::TCP),
            "UCP"  => Ok(ListenerProtocol::UDP),
            "HTTP"  => Ok(ListenerProtocol::HTTP),
            "ICMP"  => Ok(ListenerProtocol::ICMP),
            "DNS"  => Ok(ListenerProtocol::DNS),
            _      => Err(()),
        }
    }
}

impl FromSql for ListenerProtocol
{
    fn column_result(
        value: ValueRef<'_>
    ) -> FromSqlResult<Self>
    {
        match value.as_str()
        {
            Ok(value_str) => {
                match value_str
                {
                    "TCP" => Ok(ListenerProtocol::TCP),
                    "UDP" => Ok(ListenerProtocol::UDP),
                    "HTTP" => Ok(ListenerProtocol::HTTP),
                    "ICMP" => Ok(ListenerProtocol::ICMP),
                    "DNS" => Ok(ListenerProtocol::DNS),
                    _ => Err(FromSqlError::InvalidType)
                }
            },
            Err(_) => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for ImplantTaskStatus
{
    fn column_result(
        value: ValueRef<'_>
    ) -> FromSqlResult<Self>
    {
        match value.as_str()
        {
            Ok(value_str) => {
                match value_str
                {
                    "Completed" => Ok(ImplantTaskStatus::Completed),
                    "Issued" => Ok(ImplantTaskStatus::Issued),
                    "Pending" => Ok(ImplantTaskStatus::Pending),
                    _ => Err(FromSqlError::InvalidType)
                }
            },
            Err(_) => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromStr for ImplantTaskStatus
{
    type Err = ();

    fn from_str(input: &str) -> Result<ImplantTaskStatus, Self::Err> {
        match input {
            "Issued"  => Ok(ImplantTaskStatus::Issued),
            "Pending"  => Ok(ImplantTaskStatus::Pending),
            "Completed"  => Ok(ImplantTaskStatus::Completed),
            _      => Err(()),
        }
    }
}

impl HTTPListener
{
    pub fn create(address: String, port: u16) -> Result<HTTPListener, Box<dyn std::error::Error>>
    {
        let ip_address: Result<IpAddr, AddrParseError> = address.parse::<IpAddr>();
        Ok(HTTPListener
        {
            address: ip_address?,
            host: String::from("localhost"),
            port: port
        })
    }

    pub fn add_to_database(http_listener: HTTPListener) -> bool
    {
        return database::insert_http_listener(http_listener);
    }
}

impl ManageSettings for HTTPListener
{
    fn show_settings(&self)
    {
        println!("+------------+----------------------+");
        println!("|  Property  |         Value        |");
        println!("+------------+----------------------+");

        let dict: [(&str, String); 4] = [
            ("Protocol", "HTTP".to_string()),
            ("Address", self.address.to_string()),
            ("Port", self.port.to_string()),
            ("Host", self.host.to_string())
        ];

        for x in dict
        {
            println!(
                "| {0:^10} | {1:<20} |",
                x.0,
                x.1
            )
        }

        println!("+------------+----------------------+");
    }

    fn set_option(&mut self, option: &str, value: &str) -> bool
    {
        let mut flag: bool = false;

        let option_lowercase = option.to_lowercase();
        let mut option_chars: Chars = option_lowercase.chars();
        let option_capitalized: String = match option_chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + option_chars.as_str(),
        };

        // println!("[+] Setting option: {}", option_capitalized.as_str());
        match option_capitalized.as_str()
        {
            "Address" =>
            {
                // println!("[+] Setting listener address to {}", value);

                let res: Result<IpAddr, AddrParseError> = value.parse::<IpAddr>();
                if ! res.is_err()
                {
                    self.address = res.unwrap();
                    flag = true;
                }
            },
            "Port" =>
            {
                // println!("[+] Setting listener port to {}", value);

                let res: Result<u16, ParseIntError> = value.parse::<u16>();
                if ! res.is_err() 
                {
                    let port: u16 = res.unwrap();
                    if port > 0
                    {
                        self.port = port;
                        flag = true;
                    }
                }
            },
            "Host" =>
            {
                // println!("[+] Setting listener host to {}", value);

                if value.chars().count() <= 100
                {
                    self.host = value.to_string();
                    flag = true;
                }
            },
            &_ => {}
        }

        return flag;
    }
}

impl TryFrom<&Row<'_>> for HTTPListener
{
    type Error = rusqlite::Error;

    fn try_from(sql_row: &Row<'_>) -> Result<Self, Self::Error> {
        match (sql_row.get(0), sql_row.get(1), sql_row.get(2))
        {
            (Ok::<String, _>(ip_address_str), Ok::<u16, _>(listener_port), Ok::<String, _>(http_host)) => {

                match ip_address_str.parse::<IpAddr>()
                {
                    Ok(ip_address) => {
                        Ok(HTTPListener {
                            address: ip_address,
                            port: listener_port,
                            host: http_host,
                        })
                    },
                    Err(_) => Err(rusqlite::Error::InvalidQuery)
                }
            },
            (_, _, _) => Err(rusqlite::Error::InvalidQuery),
        }

    }
}
impl TryFrom<&Row<'_>> for ImplantTask {
    type Error = rusqlite::Error;

    fn try_from(sql_row: &Row<'_>) -> Result<ImplantTask, rusqlite::Error>
    {
        match (
            sql_row.get(0),
            sql_row.get(1),
            sql_row.get(2),
            sql_row.get(3),
            sql_row.get(4),
            sql_row.get(5)
        )
        {
            (
                Ok::<u64, _>(task_id),
                Ok::<u16, _>(task_implant_id),
                Ok::<String, _>(task_command),
                Ok::<u64, _>(task_datetime),
                Ok::<ImplantTaskStatus, _>(task_status),
                Ok::<Vec<u8>, _>(task_output)
            ) =>
            {
                return Ok(ImplantTask {
                    id: task_id,
                    implant_id: task_implant_id,
                    command: task_command,
                    datetime: task_datetime,
                    status: task_status,
                    output: task_output
                });
            },
            (
                Ok::<u64, _>(task_id),
                Ok::<u16, _>(task_implant_id),
                Ok::<String, _>(task_command),
                Ok::<u64, _>(task_datetime),
                Ok::<ImplantTaskStatus, _>(task_status),
                Err(_)
            ) =>
            {
                return Ok(ImplantTask {
                    id: task_id,
                    implant_id: task_implant_id,
                    command: task_command,
                    datetime: task_datetime,
                    status: task_status,
                    output: vec![]
                });
            },
            (_, _, _, _, _, _) => {
                return Err(rusqlite::Error::InvalidQuery)
            }
        };
    }
}
