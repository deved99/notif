pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("ZBus error: {0}")]
    Zbus(#[from] zbus::Error),
    #[error("Failed serializing to json: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unknown,
}
