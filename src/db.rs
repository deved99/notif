use std::env;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::Result;

const DB_PATH: &'static str = "./notif.sqlite3";

pub fn get_connection() -> Result<Connection> {
    let path_exists = get_database_path().exists();
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
            expire_timeout INTEGER,
            closed INTEGER DEFAULT 0
        )",
        (),
    )?;
    Ok(())
}

fn get_database_path() -> PathBuf {
    let path = match env::var("XDG_DATA_DIR") {
        Ok(s) => format!("{}/notif.sqlite3", s),
        Err(_) => format!("{}/.local/share/notif.sqlite3", env::var("HOME").unwrap()),
    };
    Path::new(&path).to_owned()
}
