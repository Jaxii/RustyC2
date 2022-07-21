use std::any::Any;
use std::net::{IpAddr, AddrParseError};
use std::fmt;
use std::num::ParseIntError;
use std::str::{FromStr, Chars};

use crate::database;
pub struct GenericListener
{
    pub protocol: ListenerProtocol,
    pub data: Box<dyn Any>
}

pub struct TCPListener
{
    pub state: ListenerState,
    pub address: IpAddr,
    pub port: u16
}

pub struct UDPListener
{
    pub state: ListenerState,
    pub address: IpAddr,
    pub port: u16
}

pub struct HTTPListener
{
    pub id: u16,
    pub state: ListenerState,
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

pub enum ListenerProtocol
{
    TCP,
    UDP,
    HTTP,
    ICMP,
    DNS
}

pub enum ListenerState
{
    Created,
    Running,
    Suspended
}

pub enum ImplantPlatform
{
    Windows,
    Linux,
    MacOS
}

pub trait ManageSettings
{
    fn show_settings(&self);
    fn set_option(&mut self, option: &str, value: &str) -> bool;
}

impl fmt::Display for ListenerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ListenerState::Created => "Created".fmt(f),
           ListenerState::Running => "Running".fmt(f),
           ListenerState::Suspended => "Suspended".fmt(f),
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

impl fmt::Display for ImplantPlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ImplantPlatform::Windows => "Windows".fmt(f),
            ImplantPlatform::Linux => "Linux".fmt(f),
            ImplantPlatform::MacOS => "macOS".fmt(f)
       }
    }
}

impl FromStr for ListenerState
{
    type Err = ();

    fn from_str(input: &str) -> Result<ListenerState, Self::Err> {
        match input {
            "Created"  => Ok(ListenerState::Created),
            "Running"  => Ok(ListenerState::Running),
            "Suspended"  => Ok(ListenerState::Suspended),
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

impl TCPListener
{
    const PROTOCOL: ListenerProtocol = ListenerProtocol::TCP;

    fn create(address: IpAddr, port: u16) -> TCPListener
    {
        TCPListener
        {
            state: ListenerState::Created,
            address: address,
            port: port
        }
    }
}

impl UDPListener
{
    const PROTOCOL: ListenerProtocol = ListenerProtocol::UDP;
}

impl HTTPListener
{
    const PROTOCOL: ListenerProtocol = ListenerProtocol::HTTP;

    pub fn create(address: String, port: u16) -> Result<HTTPListener, Box<dyn std::error::Error>>
    {
        let ip_address: Result<IpAddr, AddrParseError> = address.parse::<IpAddr>();
        Ok(HTTPListener
        {
            id: 0,
            state: ListenerState::Created,
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
            // ("State", self.state.to_string()),
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