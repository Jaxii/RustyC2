use std::net::{IpAddr, AddrParseError};
use std::fmt;

use crate::database;

struct TCPListener
{
    state: ListenerState,
    address: IpAddr,
    port: u16
}

struct UDPListener
{
    state: ListenerState,
    address: IpAddr,
    port: u16
}

pub struct HTTPListener
{
    pub state: ListenerState,
    pub address: IpAddr,
    pub port: u16,
    pub host: String
}

enum ListenerProtocol
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
