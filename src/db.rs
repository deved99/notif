use std::path::Path;

use rusqlite::Connection;

use crate::Result;

const DB_PATH: &'static str = "./notif.sqlite3";

pub fn get_connection() -> Result<Connection> {
    let path_exists = Path::new(DB_PATH).exists();
    let connection = Connection::open(DB_PATH)?;
    if !path_exists {
        create_tables(&connection)?;
    }
    Ok(connection)
}

pub fn create_tables(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE notifications (
            id INTEGER PRIMARY KEY NOT NULL,
            app_name TEXT,
            app_icon TEXT,
            summary TEXT,
            body TEXT,
            actions TEXT,
            hints TEXT,
            expire_timeout INTEGER
        )",
        (),
    )?;
    Ok(())
}

