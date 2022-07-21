use std::{net::IpAddr, str::FromStr};
use rusqlite::{params, Connection, Result, Statement};
use std::time::{SystemTime, SystemTimeError, Duration};

use crate::models::{HTTPListener, GenericListener, ListenerState, ListenerProtocol, GenericImplant};

pub const DB_NAME: &'static str = "db.sqlite3";

pub fn prepare_db() -> Result<()>
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    /*
    Using Class Table Inheritance + Shared Primary Key for the database
    architecture.
    This way, I'll have to add some more code, but the database
    operations should be faster (join-wise).
    */

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Listeners (
            Id          INTEGER PRIMARY KEY,
            Protocol    TEXT NOT NULL,
            State       TEXT NOT NULL
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS HttpListenerSettings (
            ListenerId  INTEGER PRIMARY KEY,
            IpAddress   VARCHAR(50),
            Port        INTEGER,
            Host        VARCHAR(100),
            FOREIGN KEY(ListenerId)
                REFERENCES Listeners(Id)
                ON DELETE CASCADE
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Implants (
            Id              INTEGER PRIMARY KEY,
            CookieHash      VARCHAR(32),
            LastSeen        INTEGER NOT NULL,
            ListenerId      INTEGER,
            FOREIGN KEY(ListenerId)
                REFERENCES Listeners(Id)
                ON DELETE CASCADE
        )",
        []
    )?;

    Ok(())
}

pub fn get_listener_address(id: u16) -> String
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<String, _> = conn.query_row(
        "SELECT IpAddress  
        FROM HttpListenerSettings
        WHERE ListenerId = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the address of the listener");
}

pub fn get_listener_port(id: u16) -> u16
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<u16, _> = conn.query_row(
        "SELECT Port
        FROM HttpListenerSettings
        WHERE ListenerId = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the port of the listener");
}

pub fn insert_http_listener(listener: HTTPListener) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let mut res = conn.execute(
        "INSERT INTO Listeners(Protocol,State)
            VALUES(?1,?2)",
        params![
            "HTTP",
            listener.state.to_string()
        ]
    );

    if !res.is_err()
    {
        let row_id: i64 = conn.last_insert_rowid();

        res = conn.execute(
            "INSERT INTO HttpListenerSettings(ListenerId,IpAddress,Port,Host)
                VALUES(?1,?2,?3,?4)",
            params![
                row_id,
                listener.address.to_string(),
                listener.port,
                listener.host
            ]
        );

        if !res.is_err()
        {
            flag = true;
        }
    }

    return flag;
}

pub fn remove_listener(listener_id: u16) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let res: Result<usize, rusqlite::Error> = conn.execute(
        "DELETE FROM Listeners
        WHERE Id=?1",
        params![
            listener_id
        ]
    );

    if !res.is_err()
    {
        if res.unwrap() != 0
        {
            flag = true;
        }   
    }

    return flag;
}

pub fn check_if_implant_in_db(implant_cookie_hash: &str) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<String, _> = conn.query_row(
        "SELECT CookieHash
        FROM Implants
        WHERE CookieHash = ?1",
        params![implant_cookie_hash],
        |row| row.get(0),
    );

    if query_result.is_ok()
    {
        if query_result.unwrap() == implant_cookie_hash
        {
            flag = true;
        }   
    }

    return flag;
}

pub fn add_implant(listener_id: u16, implant_cookie_hash: &str) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let time_elapsed_now: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    if time_elapsed_now.is_err()
    {
        return flag;
    }

    println!("[+] Last seen: {}", time_elapsed_now.as_ref().unwrap().as_secs());

    let res: Result<usize, rusqlite::Error> = conn.execute(
        "INSERT INTO Implants(CookieHash,LastSeen,ListenerId)
            VALUES(?1,?2,?3)",
        params![
            implant_cookie_hash,
            time_elapsed_now.unwrap().as_secs(),
            listener_id
        ]
    );

    if res.is_ok()
    {
        if res.unwrap() == 1
        {
            flag = true;
        }
    }

    return flag;
}

pub fn get_listeners() -> Vec<GenericListener>
{
    let mut listeners: Vec<GenericListener> = Vec::new();
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = conn.prepare(
        "SELECT Id, Protocol, State, IpAddress, Port, Host
        FROM Listeners
        INNER JOIN HttpListenerSettings
            ON Listeners.Id = HttpListenerSettings.ListenerId
        ").unwrap();

    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let id: u16 = row.get(0).unwrap();
        let protocol: String = row.get(1).unwrap();
        let listener_protocol: ListenerProtocol = ListenerProtocol::from_str(protocol.as_str()).unwrap();
        let state: String = row.get(2).unwrap();
        let listener_state: ListenerState = ListenerState::from_str(state.as_str()).unwrap();
        let address: String = row.get(3).unwrap();
        let listener_address: IpAddr = address.parse::<IpAddr>().unwrap();
        let port: u16 = row.get(4).unwrap();
        let host: String = row.get(5).unwrap();

        if let ListenerProtocol::HTTP = listener_protocol
        {
            let listener: HTTPListener = HTTPListener
            {
                id: id,
                state: listener_state,
                address: listener_address,
                host: host,
                port: port
            };

            let generic_listener: GenericListener = GenericListener
            {
                protocol: ListenerProtocol::HTTP,
                data: Box::new(listener)
            };

            listeners.push(generic_listener);
        }
    }

    return listeners;
}

pub fn get_implants() -> Vec<GenericImplant>
{
    let mut implants: Vec<GenericImplant> = Vec::new();
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = conn.prepare(
        "SELECT Id, ListenerId, LastSeen
        FROM Implants
    ").unwrap();

    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let implant_id: u16 = row.get(0).unwrap();
        let listener_id: u16 = row.get(1).unwrap();
        let last_seen: u64 = row.get(2).unwrap();

        let generic_implant: GenericImplant = GenericImplant
        {
            id: implant_id,
            listener_id: listener_id,
            last_seen: last_seen,
            data: Box::new(0)
        };

        implants.push(generic_implant);
    }

    return implants;
}

pub fn remove_implant(implant_id: u16) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let res: Result<usize, rusqlite::Error> = conn.execute(
        "DELETE FROM Implants
        WHERE Id=?1",
        params![
            implant_id
        ]
    );

    if !res.is_err()
    {
        if res.unwrap() != 0
        {
            flag = true;
        }   
    }

    return flag;
}