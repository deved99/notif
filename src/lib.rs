pub mod db;
mod error;
mod notification;
mod server;

pub use error::{Error, Result};
pub use notification::Notification;
pub use server::Server;

use clap::Parser;

/// A simple notification server, which saves notification in an SQLite.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Start the notification server.
    Server,
    /// Close a notifications
    Close { id: u32 },
    /// Start the notification server.
    Read,
}
impl Command {
    async fn run(&self) -> Result<()> {
        match self {
            Self::Server => Server::new().start().await,
            Self::Close { id } => {
                let connection = zbus::Connection::session().await?;
                connection.call_method(
                    Some("org.freedesktop.Notifications"),
                    "/org/freedesktop/Notifications",
                    Some("org.freedesktop.Notifications"),
                    "CloseNotification",
                    &(id, 2 as u32),
                ).await?;
                Ok(())
            },
            Self::Read => {
                let notifications = Notification::list()?;
                let json = serde_json::to_string(&notifications)?;
                println!("{}", json);
                Ok(())
            }
        }
    }
}

pub async fn main() -> Result<()> {
    let clap = Cli::parse();
    clap.command.run().await
}
