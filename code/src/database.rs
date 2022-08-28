use std::{net::IpAddr, str::FromStr};
use chrono::format::{DelayedFormat, StrftimeItems};
use rusqlite::{params, Connection, Result, Statement, ToSql, params_from_iter};
use std::time::{SystemTime, SystemTimeError, Duration};

use crate::models::{HTTPListener, GenericListener, ListenerStatus, ListenerProtocol, GenericImplant, self, ImplantTask, ImplantTaskStatus};
use crate::misc;

pub const DB_NAME: &'static str = "db.sqlite3";

pub fn prepare_db() -> Result<()>
{
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    /*
    Using Class Table Inheritance + Shared Primary Key for the database
    architecture.
    This way, I'll have to add some more code, but the database
    operations should be faster (join-wise).
    */

    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS Listeners (
            Id          INTEGER PRIMARY KEY,
            Protocol    TEXT NOT NULL,
            Status      TEXT NOT NULL
        )",
        []
    )?;

    db_connection.execute(
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

    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS Implants (
            Id              INTEGER PRIMARY KEY,
            CookieHash      VARCHAR(32) UNIQUE,
            LastSeen        INTEGER NOT NULL,
            ListenerId      INTEGER,
            FOREIGN KEY(ListenerId)
                REFERENCES Listeners(Id)
                ON DELETE CASCADE
        )",
        []
    )?;

    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS ImplantTasks (
            Id              INTEGER PRIMARY KEY,
            ImplantId       INTEGER NOT NULL,
            Command         TEXT NOT NULL,
            DateTime        INTEGER NOT NULL,
            Status          VARCHAR(30) NOT NULL,
            Output          BLOB,
            FOREIGN KEY(ImplantId)
                REFERENCES Implants(Id)
                ON DELETE CASCADE
        )",
        []
    )?;

    db_connection.execute(
        "UPDATE Listeners
        SET Status = ?1
        WHERE Status = ?2
        ",
        params![
            ListenerStatus::Suspended.to_string(),
            ListenerStatus::Active.to_string()
        ]
    )?;

    Ok(())
}

pub fn get_listener_address(id: u16) -> String
{
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<String, _> = db_connection.query_row(
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let query_result: Result<u16, _> = db_connection.query_row(
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let mut res = db_connection.execute(
        "INSERT INTO Listeners(Protocol,Status)
            VALUES(?1, ?2)",
        params![
            "HTTP",
            models::ListenerStatus::Created.to_string()
        ]
    );

    if !res.is_err()
    {
        let row_id: i64 = db_connection.last_insert_rowid();

        res = db_connection.execute(
            "INSERT INTO HttpListenerSettings(ListenerId,IpAddress,Port,Host)
                VALUES(?1, ?2, ?3, ?4)",
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "DELETE FROM Listeners
        WHERE Id = ?1",
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

pub fn check_if_implant_exists(
    implant_id: Option<u16>,
    implant_cookie_hash: Option<&str>
) -> Option<u16>
{
    let db_connection = Connection::open(DB_NAME);

    if db_connection.is_err()
    {
        return None;
    }

    if implant_id.is_some()
    {
        let query_result: Result<u16, _> = db_connection.unwrap().query_row(
            "SELECT Id
            FROM Implants
            WHERE Id = ?1",
            params![implant_id.unwrap()],
            |row| row.get(0),
        );

        return match query_result
        {
            Ok(v) => {
                Some(v)
            },
            Err(_) => {
                None
            }
        }
    }  
    else if implant_cookie_hash.is_some()
    {
        let query_result: Result<u16, _> = db_connection.unwrap().query_row(
            "SELECT Id
            FROM Implants
            WHERE CookieHash = ?1",
            params![implant_cookie_hash.unwrap()],
            |row| row.get(0),
        );

        return match query_result
        {
            Ok(v) => {
                Some(v)
            },
            Err(_) => {
                None
            }
        }
    };

    return None;
}

pub fn add_implant(listener_id: u16, implant_cookie_hash: &str) -> bool
{
    let mut flag: bool = false;
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let time_elapsed_now: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    if time_elapsed_now.is_err()
    {
        return flag;
    }

    let last_seen_unix_timestamp = time_elapsed_now.as_ref().unwrap().as_secs();
    let formatted_last_seen: DelayedFormat<StrftimeItems> = misc::utils::format_date_time(
        last_seen_unix_timestamp,
        "%Y-%m-%d %H:%M:%S"
    );
    println!("[+] Last seen: {}", formatted_last_seen);

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "INSERT INTO Implants(CookieHash,LastSeen,ListenerId)
            VALUES(?1, ?2, ?3)",
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = db_connection.prepare(
        "SELECT Id, Protocol, Status, IpAddress, Port, Host
        FROM Listeners
        INNER JOIN HttpListenerSettings
            ON Listeners.Id = HttpListenerSettings.ListenerId
        ").unwrap();

    let mut rows = statement.query([]).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let listener_id: u16 = row.get(0).unwrap();
        let protocol: String = row.get(1).unwrap();
        let listener_protocol: ListenerProtocol = ListenerProtocol::from_str(protocol.as_str()).unwrap();
        let status_string: String = row.get(2).unwrap();
        let listener_status: ListenerStatus = ListenerStatus::from_str(status_string.as_str()).unwrap();
        let address: String = row.get(3).unwrap();
        let listener_address: IpAddr = address.parse::<IpAddr>().unwrap();
        let port: u16 = row.get(4).unwrap();
        let host: String = row.get(5).unwrap();

        if let ListenerProtocol::HTTP = listener_protocol
        {
            let listener: HTTPListener = HTTPListener
            {
                address: listener_address,
                host: host,
                port: port
            };

            let generic_listener: GenericListener = GenericListener
            {
                id: listener_id,
                protocol: ListenerProtocol::HTTP,
                status: listener_status,
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let mut statement: Statement = db_connection.prepare(
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
    let db_connection: Connection = Connection::open(DB_NAME).unwrap();

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "DELETE FROM Implants
        WHERE Id = ?1",
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

pub fn set_listener_status(listener_id: u16, listener_status: ListenerStatus) -> bool
{
    let mut flag: bool = false;
    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return false;
    }

    let db_connection: Connection = conn_result.unwrap();

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "UPDATE Listeners
        SET Status = ?1
        WHERE Id = ?2",
        params![
            listener_status.to_string(),
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
    else
    {
        println!("{}", res.unwrap());
    }
    
    return flag;
}

pub fn update_implant_timestamp(implant_cookie_hash: &str) -> bool
{
    let mut flag: bool = false;
    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return flag;
    }

    let db_connection: Connection = conn_result.unwrap();

    let time_elapsed_now: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    if time_elapsed_now.is_err()
    {
        return flag;
    }

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "UPDATE Implants
        SET LastSeen = ?1
        WHERE CookieHash = ?2",
        params![
            time_elapsed_now.unwrap().as_secs(),
            implant_cookie_hash
        ]
    );

    if res.is_ok()
    {
        if res.unwrap() == 1
        {
            flag = true;
        }
    }
    else
    {
        println!("{}", res.unwrap());
    }
    
    return flag;
}

pub fn create_implant_task(implant_id: u16, task_name: &str) -> bool
{
    let mut flag: bool = false;
    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return flag;
    }

    let db_connection: Connection = conn_result.unwrap();

    let time_elapsed_now: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    if time_elapsed_now.is_err()
    {
        return flag;
    }

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "INSERT INTO ImplantTasks(ImplantId, Command, DateTime, Status)
        VALUES (?1, ?2, ?3, ?4)",
        params![
            implant_id,
            task_name,
            time_elapsed_now.unwrap().as_secs(),
            ImplantTaskStatus::Issued.to_string()
        ]
    );

    if res.is_ok()
    {
        if res.unwrap() == 1
        {
            flag = true;
        }
    }
    else
    {
        println!("{}", res.unwrap());
    }
    
    return flag;
}

pub fn get_all_tasks(ignore_completed: bool) -> Vec<ImplantTask>
{
    let mut tasks: Vec<ImplantTask> = Vec::new();

    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return tasks;
    }

    let db_connection: Connection = conn_result.unwrap();

    let mut sql_statement: String = "SELECT Id, ImplantId, Command, DateTime, Status, Output
        FROM ImplantTasks ".to_string();

    let mut where_condition: String = String::new();
    let mut vec_params: Vec<String> = Vec::new(); 

    if ignore_completed
    {
        where_condition = "WHERE Status != ?".to_string();
        vec_params.push(ImplantTaskStatus::Completed.to_string());
    }

    sql_statement.push_str(where_condition.as_str());
    let mut statement: Statement = db_connection.prepare(sql_statement.as_str()).unwrap();

    let sql_params: Vec<_> = vec_params.iter().map(|x| x as &dyn ToSql).collect();

    let mut rows = statement.query(&*sql_params).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let task_id: u64 = row.get(0).unwrap();
        let implant_id: u16 = row.get(1).unwrap();
        let task_command: String = row.get(2).unwrap();
        let task_date_time: u64 = row.get(3).unwrap();
        let status: String = row.get(4).unwrap();
        let task_status: ImplantTaskStatus = ImplantTaskStatus::from_str(status.as_str()).unwrap();
        let task_output: Vec<u8> = match row.get(5)
        {
            Ok(v) => v,
            Err(_) => vec![]
        };
        
        let implant_task: ImplantTask = ImplantTask {
            id: task_id,
            implant_id: implant_id,
            command: task_command,
            datetime: task_date_time,
            status: task_status,
            output: task_output
        };

        tasks.push(implant_task);
    }

    return tasks;
}

pub fn get_implant_tasks(
    implant_identifier_name: &str,
    implant_identifier_value: &str,
    include_statuses: Vec<String>
) -> Vec<ImplantTask>
{
    let mut tasks: Vec<ImplantTask> = Vec::new();
    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return tasks;
    }
    
    let db_connection: Connection = conn_result.unwrap();
    
    let mut sql_statement: String = "SELECT ImplantTasks.Id, ImplantId, Command, DateTime, ImplantTasks.Status, Output
        FROM ImplantTasks
        JOIN Implants
        ON ImplantTasks.ImplantId = Implants.Id
        WHERE Implants.{{ COLUMN_IDENTIFIER }} = ?
            AND ImplantTasks.Status IN ({{ STATUS_PARAMETERS }})
        ORDER BY ImplantTasks.DateTime DESC
        ".to_string().replace("{{ COLUMN_IDENTIFIER }}", implant_identifier_name);

    let mut vec_params: Vec<String> = Vec::new();
    vec_params.push(implant_identifier_value.to_string());
    
    let mut status_parameters: String = "?,".repeat(include_statuses.len());
    
    if include_statuses.len() > 0
    {
        assert!(status_parameters.pop().unwrap() == ",".chars().next().unwrap());
        sql_statement = sql_statement.replace("{{ STATUS_PARAMETERS }}", status_parameters.as_str());

        for status in include_statuses
        {
            vec_params.push(status);
        }
    }

    let sql_params: Vec<_> = vec_params.iter().map(|x| x as &dyn ToSql).collect();
    let mut statement: Statement = db_connection.prepare(&sql_statement).unwrap();
    let mut rows = statement.query(params_from_iter(sql_params.iter())).unwrap();

    while let Some(row) = rows.next().unwrap()
    {
        let task_id: u64 = row.get(0).unwrap();
        let implant_id: u16 = row.get(1).unwrap();
        let task_command: String = row.get(2).unwrap();
        let task_date_time: u64 = row.get(3).unwrap();
        let status: String = row.get(4).unwrap();
        let task_status: ImplantTaskStatus = ImplantTaskStatus::from_str(status.as_str()).unwrap();
        let task_output: Vec<u8> = match row.get(5)
        {
            Ok(v) => v,
            Err(_) => vec![]
        };
        
        let implant_task: ImplantTask = ImplantTask {
            id: task_id,
            implant_id: implant_id,
            command: task_command,
            datetime: task_date_time,
            status: task_status,
            output: task_output
        };

        tasks.push(implant_task);
    }
    
    return tasks;
}

pub fn update_implant_task_status(
    task_id: u64,
    new_status: ImplantTaskStatus
) -> bool
{
    let mut flag: bool = false;
    let conn_result: Result<Connection, _> = Connection::open(DB_NAME);
    
    if conn_result.is_err()
    {
        return false;
    }

    let db_connection: Connection = conn_result.unwrap();

    let res: Result<usize, rusqlite::Error> = db_connection.execute(
        "UPDATE ImplantTasks
        SET Status = ?1
        WHERE Id = ?2",
        params![
            new_status.to_string(),
            task_id
        ]
    );

    if res.is_ok()
    {
        if res.unwrap() == 1
        {
            flag = true;
        }
    }
    else
    {
        println!("{}", res.unwrap());
    }
    
    return flag;
}

fn get_implant_task_last_issued(
    implant_id: u16
) -> u64
{
    let mut task_id: u64 = 0;

    match Connection::open(DB_NAME)
    {
        Ok(db_connection) => {
            let query_result: Result<u64, _> = db_connection.query_row(
                "SELECT ImplantTasks.Id
                FROM ImplantTasks
                WHERE ImplantId = ?1 AND ImplantTasks.Status = ?2
                ORDER BY ImplantTasks.DateTime ASC
                LIMIT 1",
                params![
                    implant_id,
                    ImplantTaskStatus::Pending.to_string()
                ],
                |row| row.get(0),
            );
    
            match query_result
            {
                Ok(db_task_id) => {
                    task_id = db_task_id;
                },
                Err(_) => {}
            }
        },
        Err(_) => {}
    }

    return task_id;
}

pub fn update_implant_task_output(
    implant_id: u16,
    task_output_bytes: &[u8]
) -> bool
{
    let mut flag: bool = false;
    
    let task_id = get_implant_task_last_issued(implant_id);
    if task_id == 0
    {
        return false;
    }

    match Connection::open(DB_NAME)
    {
        Ok(db_connection) => {

            let res: Result<usize, rusqlite::Error> = db_connection.execute(
                "UPDATE ImplantTasks
                SET Output = ?1, Status = ?2
                WHERE Id = ?3",
                params![
                    task_output_bytes,
                    ImplantTaskStatus::Completed.to_string(),
                    task_id
                ]
            );

            match res
            {
                Ok(v) => {
                    if v == 1
                    {
                        flag = true;
                    }
                },
                Err(_) => {}
            }
        },
        Err(_) => 
        {
            return false;
        }
    }

    return flag;
}