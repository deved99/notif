use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type};

use crate::{db, utils, Error, Result};

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
}
