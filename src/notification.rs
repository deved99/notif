use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type};

use crate::{db, utils, Result};

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
