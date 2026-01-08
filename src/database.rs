use chrono::Utc;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Database {
    conn: Connection,
    path: String,
    version: String,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).expect("Failed to open database connection.");
        let version = conn
            .query_row("SELECT sqlite_version()", [], |row| row.get(0))
            .expect("Failed to get SQLite version.");

        Self {
            conn,
            path: path.to_string(),
            version,
        }
    }

    pub fn init(&self) -> Result<()> {
        self.create_tables().expect("Error initializing SQLite tables");
        Ok(())
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            value INTEGER NOT NULL,
            type INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn insert_entry(&self, value: u16) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO entries (timestamp, value) VALUES  (?1, ?2)",
            [timestamp, value as i64],
        )?;
        Ok(())
    }

    pub fn get_data_path(app_name: &str) -> std::io::Result<PathBuf> {
        let base_dir = dirs::data_local_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine data directory"))?;

        let app_dir = base_dir.join(app_name);
        let _ = fs::create_dir_all(&app_dir);
        let db_path = app_dir.join(format!("{}.db3", app_name));

        Ok(db_path)
    }
}
