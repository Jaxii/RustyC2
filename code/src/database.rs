use std::{net::IpAddr, str::FromStr};
use rusqlite::{params, Connection, Result, Statement};

use crate::models::{HTTPListener, Listener, ListenerState, ListenerProtocol};

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
            FOREIGN KEY(ListenerId) REFERENCES Listeners(Id) ON DELETE CASCADE
        )",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Implants (
            Id              INTEGER PRIMARY KEY,
            LastSeen        INTEGER NOT NULL,
            ListenerId      INTEGER,
            FOREIGN KEY(ListenerId) REFERENCES Listeners(Id) ON DELETE CASCADE
        )",
        []
    )?;

    Ok(())
}

pub fn get_listener_address(id: u16) -> String
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<String, _> = conn.query_row(
        "SELECT address FROM Listeners WHERE Id = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the address of the listener");
}

pub fn get_listener_port(id: u16) -> u16
{
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<u16, _> = conn.query_row(
        "SELECT port FROM Listeners WHERE Id = ?1",
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
            "INSERT INTO HttpListenerSettings(ListenerId,IpAddress,Port)
                VALUES(?1,?2,?3)",
            params![
                row_id,
                listener.address.to_string(),
                listener.port
            ]
        );

        if !res.is_err()
        {
            flag = true;
        }
    }

    return flag;
}

pub fn get_listeners() -> Vec<Listener>
{
    let mut listeners: Vec<Listener> = Vec::new();
    let conn: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = conn.prepare(
        "SELECT Id, Protocol, State FROM Listeners").unwrap();
    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let id: u16 = row.get(0).unwrap();
        let protocol: String = row.get(1).unwrap();
        let state: String = row.get(2).unwrap();

        // println!("{:?}", port);
        // println!("{:?}", address);
        // println!("{:?}", protocol);
        // println!("{:?}", state);

        let listener = Listener
        {
            id: id,
            protocol: ListenerProtocol::from_str(protocol.as_str()).unwrap(),
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
        "DELETE FROM Listeners
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
