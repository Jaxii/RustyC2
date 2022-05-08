use rusqlite::{Connection, Result};

pub fn prepare_db() -> Result<()>
{
    let conn = Connection::open("db.sqlite3").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS listeners (
            id    INTEGER PRIMARY KEY,
            protocol TEXT NOT NULL,
            state  TEXT NOT NULL,
            port INTEGER
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