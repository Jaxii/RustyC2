use std::any::Any;
use std::net::{IpAddr, AddrParseError};
use std::fmt;
use std::num::ParseIntError;
use std::str::{FromStr, Chars};

use crate::database;
pub struct GenericListener
{
    pub id: u16,
    pub protocol: ListenerProtocol,
    pub status: ListenerStatus,
    pub data: Box<dyn Any>
}

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
    pub output: String
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