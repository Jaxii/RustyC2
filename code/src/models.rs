use std::net::{IpAddr, AddrParseError};
use std::fmt;
use std::str::FromStr;

use crate::database;

pub struct Listener
{
    pub state: ListenerState,
    pub address: IpAddr,
    pub port: u16,
    pub protocol: ListenerProtocol,
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

impl fmt::Display for ListenerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ListenerState::Created => write!(f, "Created"),
           ListenerState::Running => write!(f, "Running"),
           ListenerState::Suspended => write!(f, "Suspended"),
       }
    }
}

impl fmt::Display for ListenerProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ListenerProtocol::TCP => write!(f, "TCP"),
            ListenerProtocol::UDP => write!(f, "UDP"),
            ListenerProtocol::HTTP => write!(f, "HTTP"),
            ListenerProtocol::ICMP => write!(f, "ICMP"),
            ListenerProtocol::DNS => write!(f, "DNS"),
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

    pub fn create(address: String, port: u16) -> bool
    {
        let mut flag: bool = false;

        let ip_address: Result<IpAddr, AddrParseError> = address.parse::<IpAddr>();
        if ! ip_address.is_err()
        {
            let http_listener: HTTPListener = HTTPListener
            {
                state: ListenerState::Created,
                address: ip_address.unwrap(),
                host: String::from("localhost"),
                port: port
            };

            flag = database::insert_http_listener(http_listener);
        }

        return flag;
    }
}
