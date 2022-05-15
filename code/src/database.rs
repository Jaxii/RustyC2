use rusqlite::{params, Connection, Result};

use crate::models::HTTPListener;

pub const db_name: &'static str = "db.sqlite3";

pub fn prepare_db() -> Result<()>
{
    let conn = Connection::open(db_name).unwrap();

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

pub fn get_next_id() -> u16
{
    let conn = Connection::open(db_name).unwrap();

    let query_result: Result<u16, _> = conn.query_row(
        "SELECT COUNT(*) AS ListenersCounter FROM listeners",
        [],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the ID for the listener");
}

pub fn get_listener_address(id: u16) -> String
{
    let conn = Connection::open(db_name).unwrap();

    let query_result: Result<String, _> = conn.query_row(
        "SELECT address FROM listeners WHERE Id = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the address of the listener");
}

pub fn get_listener_port(id: u16) -> u16
{
    let conn = Connection::open(db_name).unwrap();

    let query_result: Result<u16, _> = conn.query_row(
        "SELECT port FROM listeners WHERE Id = ?1",
        params![id],
        |row| row.get(0),
    );

    return query_result.expect("[!] Couldn't retrieve the port of the listener");
}

pub fn insert_http_listener(listener: HTTPListener) -> bool
{
    let flag = false;
    let conn = Connection::open(db_name).unwrap();

    conn.execute(
        "INSERT INTO listeners(protocol,address,port,state)
            VALUES(?1,?2,?3,?4)",
        params![
            "HTTP",
            listener.address.to_string(),
            listener.port,
            listener.state.to_string()
        ]
    );

    return flag;
}
