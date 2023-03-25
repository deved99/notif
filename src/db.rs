pub fn create() -> String {
    "CREATE TABLE notifications (
        id INTEGER PRIMARY KEY NOT NULL,
        app_name TEXT,
        app_icon TEXT,
        summary TEXT,
        body TEXT,
        actions TEXT,
        hints TEXT,
        expire_timeout INTEGER
    )".to_string()
}
