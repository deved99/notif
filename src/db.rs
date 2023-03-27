use std::env;
use std::path::Path;

use rusqlite::Connection;

use crate::Result;

pub fn get_connection() -> Result<Connection> {
    let path = get_database_path();
    let path_exists = Path::new(&path).exists();
    let connection = Connection::open(&path)?;
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
            expire_timeout INTEGER,
            closed INTEGER DEFAULT 0
        )",
        (),
    )?;
    Ok(())
}

fn get_database_path() -> String {
    match env::var("XDG_DATA_DIR") {
        Ok(s) => format!("{}/notif.sqlite3", s),
        Err(_) => format!("{}/.local/share/notif.sqlite3", env::var("HOME").unwrap()),
    }
}
