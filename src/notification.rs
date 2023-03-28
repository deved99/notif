use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type};

use crate::{db, Error, Result};

#[derive(Type, Deserialize, Serialize)]
pub struct Notification {
    app_name: String,
    id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: HashMap<String, OwnedValue>,
    expire_timeout: i32,
}

impl Notification {
    pub fn list() -> Result<Vec<Self>> {
        let db = db::get_connection()?;
        let mut stmt = db.prepare(
            "SELECT id, app_name, app_icon, summary, body, actions, hints, expire_timeout 
            FROM notifications
            WHERE closed = 0",
        )?;
        let notifications = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<usize, i32>(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get::<usize, String>(5)?,
                    row.get::<usize, String>(6)?,
                    row.get(7)?,
                ))
            })?
            .map(|x| {
                x.map_err(Error::from).and_then(
                    |(id, app_name, app_icon, summary, body, actions, hints, expire_timeout)| {
                        Ok(Self {
                            id: id as u32,
                            app_name,
                            app_icon,
                            summary,
                            body,
                            actions: serde_json::from_str(&actions)?,
                            hints: serde_json::from_str(&hints)?,
                            expire_timeout,
                        })
                    },
                )
            })
            .collect();
        notifications
    }
    pub fn save(&self) -> Result<u32> {
        let db = db::get_connection()?;
        let actions = serde_json::to_string(&self.actions)?;
        let hints = serde_json::to_string(&self.hints)?;
        let results_maybe: Result<Vec<u32>> = match self.id {
            0 => {
                let mut query = db.prepare(
                    "INSERT INTO notifications
                    (app_name, app_icon, summary, body, actions, hints, expire_timeout)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    RETURNING id",
                )?;
                let results: Result<Vec<u32>> = query
                    .query_map(
                        [
                            &self.app_name,
                            &self.app_icon,
                            &self.summary,
                            &self.body,
                            &actions,
                            &hints,
                            &self.expire_timeout.to_string(),
                        ],
                        |row| row.get::<usize, i32>(0),
                    )?
                    .map(|x| x.map_err(Error::from))
                    .map(|x| x.map(|n| n as u32))
                    .collect();
                results
            }
            x => {
                let mut query = db.prepare(
                    "INSERT INTO notifications
                    (id, app_name, app_icon, summary, body, actions, hints, expire_timeout)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(id) DO UPDATE SET
                        app_name = excluded.app_name, 
                        app_icon = excluded.app_icon, 
                        summary = excluded.summary, 
                        body = excluded.body, 
                        actions = excluded.actions, 
                        hints = excluded.hints, 
                        expire_timeout = excluded.expire_timeout
                    RETURNING id",
                )?;
                let results: Result<Vec<u32>> = query
                    .query_map(
                        [
                            &(x as i32).to_string(),
                            &self.app_name,
                            &self.app_icon,
                            &self.summary,
                            &self.body,
                            &actions,
                            &hints,
                            &self.expire_timeout.to_string(),
                        ],
                        |row| row.get::<usize, i32>(0),
                    )?
                    .map(|x| x.map_err(Error::from))
                    .map(|x| x.map(|n| n as u32))
                    .collect();
                results
            }
        };
        let results = results_maybe?;
        if results.len() != 1 {
            panic!("I expected to be inserting exactly one row: {:?}", results);
        }
        Ok(results[0])
    }
    pub fn close(id: u32, reason: u32) -> Result<()> {
        let db = db::get_connection()?;
        db.execute("UPDATE notifications SET closed = ? WHERE id = ?", (reason, id as i32))
            .map_err(Error::from)
            .map(|_| ())
    }
}
