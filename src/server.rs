use crate::Notification;

use serde::{Deserialize, Serialize};
use zbus::dbus_interface;
use zvariant::Type;

#[derive(Type, Deserialize, Serialize)]
pub struct Server {
    name: String,
    vendor: String,
    version: String,
    spec_version: String,
}

impl Server {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            vendor: env!("CARGO_PKG_AUTHORS").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            spec_version: "Uhhhh".to_string(),
        }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Server {
    async fn notify(&self, n: Notification) -> u32 {
        let json = serde_json::to_string(&n).unwrap();
        println!("{}", json);
        1
    }
    async fn get_server_information(&self) -> &Server {
        self
    }
}
