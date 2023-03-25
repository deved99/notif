pub mod db;
mod error;
mod notification;
mod server;
mod utils;

use zbus::Connection;

pub use error::{Error, Result};
pub use notification::Notification;
pub use server::Server;

pub async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    // Setup the server
    connection
        .object_server()
        .at("/org/freedesktop/Notifications", Server::new())
        .await?;
    // And request the name
    connection
        .request_name("org.freedesktop.Notifications")
        .await?;

    // Then wait while zbus manages dbus calls
    std::future::pending::<()>().await;
    Ok(())
}
