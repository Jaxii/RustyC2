use std::any::Any;
use std::net::{IpAddr, AddrParseError};
use std::fmt;
use std::str::FromStr;

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

pub trait ShowSettings
{
    fn show_settings(&self);
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

impl ShowSettings for HTTPListener
{
    fn show_settings(&self)
    {
        println!("+------------+----------------------+");
        println!("|  Property  |         Value        |");
        println!("+------------+----------------------+");

        let dict: [(&str, String); 5] = [
            ("State", self.state.to_string()),
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
}