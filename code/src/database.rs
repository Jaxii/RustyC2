use std::{net::IpAddr, str::FromStr};

use rusqlite::{params, Connection, Result, Statement};

use crate::models::{HTTPListener, Listener, ListenerState, ListenerProtocol};

pub const DB_NAME: &'static str = "db.sqlite3";

pub fn prepare_db() -> Result<()>
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS listeners (
            id    INTEGER PRIMARY KEY,
            protocol TEXT NOT NULL,
            address VARCHAR(50),
            port INTEGER,
            state  TEXT NOT NULL
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS implants (
            id    INTEGER PRIMARY KEY,
            last_seen  INTEGER NOT NULL,
            listener_id INTEGER,
            FOREIGN KEY(listener_id) REFERENCES listeners(id)
        )",
        []
    )?;

    Ok(())
}

pub fn get_listener_address(id: u16) -> String
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<String, _> = conn.query_row(
        "SELECT address FROM listeners WHERE Id = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the address of the listener");
}

pub fn get_listener_port(id: u16) -> u16
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<u16, _> = conn.query_row(
        "SELECT port FROM listeners WHERE Id = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the port of the listener");
}

pub fn insert_http_listener(listener: HTTPListener) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let res = conn.execute(
        "INSERT INTO listeners(protocol,address,port,state)
            VALUES(?1,?2,?3,?4)",
        params![
            "HTTP",
            listener.address.to_string(),
            listener.port,
            listener.state.to_string()
        ]
    );
    if !res.is_err()
    {
        flag = true;
    }

    return flag;
}

pub fn get_listeners() -> Vec<Listener>
{
    let mut listeners: Vec<Listener> = Vec::new();
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = conn.prepare(
        "SELECT id, protocol, address, port, state FROM listeners").unwrap();
    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let id: u16 = row.get(0).unwrap();
        let protocol: String = row.get(1).unwrap();
        let address_string: String = row.get(2).unwrap();
        let address: IpAddr = address_string.parse::<IpAddr>().unwrap();
        let port: u16 = row.get(3).unwrap();
        let state: String = row.get(4).unwrap();

        // println!("{:?}", port);
        // println!("{:?}", address);
        // println!("{:?}", protocol);
        // println!("{:?}", state);

        let listener = Listener
        {
            id: id,
            protocol: ListenerProtocol::from_str(protocol.as_str()).unwrap(),
            port: port,
            address: address,
            state: ListenerState::from_str(state.as_str()).unwrap()
        };

        listeners.push(listener);
    }

    return listeners;
}

pub fn remove_listener(listener_id: u16) -> bool
{
    let mut flag: bool = false;
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let res: Result<usize, rusqlite::Error> = conn.execute(
        "DELETE FROM listeners
            WHERE id=?1",
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
