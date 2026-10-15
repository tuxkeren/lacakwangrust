//! Lapisan database.
//!
//! Padanan dari `config/Database.php` pada versi PHP, memakai `rusqlite`.

pub mod schema;

use rusqlite::Connection;
use std::path::Path;

/// Pembungkus koneksi SQLite tunggal (aplikasi desktop single-user).
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Membuka (atau membuat) file database SQLite.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        let conn = Connection::open(path)?;
        // Sama seperti versi PHP: aktifkan foreign key + resolusi ON DELETE.
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Self { conn })
    }

    /// Referensi ke koneksi (semua operasi rusqlite memakai `&self`).
    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
