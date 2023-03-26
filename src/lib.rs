pub mod db;
mod error;
mod notification;
mod server;
mod utils;

pub use error::{Error, Result};
pub use notification::Notification;
pub use server::Server;

pub async fn main() -> Result<()> {
    server::Server::new().start().await
}
