use crate::{Notification, Result};

use serde::{Deserialize, Serialize};
use zbus::{dbus_interface, Connection};
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
    pub async fn start(self) -> Result<()> {
        let connection = Connection::session().await?;
        // Setup the server
        connection
            .object_server()
            .at("/org/freedesktop/Notifications", self)
            .await?;
        // And request the name
        connection
            .request_name("org.freedesktop.Notifications")
            .await?;

        // Then wait while zbus manages dbus calls
        std::future::pending::<()>().await;
        Ok(())
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Server {
    async fn notify(&self, n: Notification) -> u32 {
        n.save().unwrap()
    }
    async fn get_server_information(&self) -> &Server {
        self
    }
}
