//! Model `Category` — padanan dari `models/Category.php`.

use rusqlite::{params, Connection, Row};

#[derive(Clone, Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
    /// `"income"` atau `"expense"`.
    pub kind: String,
    pub color: String,
    /// Kolom `created_at` pada tabel `categories`.
    #[allow(dead_code)]
    pub created_at: String,
}

impl Category {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            kind: row.get("type")?,
            color: row.get("color")?,
            created_at: row.get::<_, Option<String>>("created_at")?.unwrap_or_default(),
        })
    }

    /// Semua kategori, diurutkan berdasarkan nama.
    pub fn all(conn: &Connection) -> rusqlite::Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT * FROM categories ORDER BY name ASC")?;
        let rows = stmt.query_map([], Self::from_row)?;
        rows.collect()
    }

    /// Kategori dengan tipe tertentu (`income` / `expense`).
    pub fn by_type(conn: &Connection, kind: &str) -> rusqlite::Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT * FROM categories WHERE type = ?1 ORDER BY name ASC")?;
        let rows = stmt.query_map(params![kind], Self::from_row)?;
        rows.collect()
    }

    pub fn get(conn: &Connection, id: i64) -> rusqlite::Result<Option<Self>> {
        let mut stmt = conn.prepare("SELECT * FROM categories WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], Self::from_row)?;
        match rows.next() {
            Some(Ok(cat)) => Ok(Some(cat)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    pub fn create(
        conn: &Connection,
        name: &str,
        kind: &str,
        color: &str,
    ) -> rusqlite::Result<i64> {
        conn.execute(
            "INSERT INTO categories (name, type, color) VALUES (?1, ?2, ?3)",
            params![name, kind, color],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update(conn: &Connection, id: i64, name: &str, color: &str) -> rusqlite::Result<()> {
        conn.execute(
            "UPDATE categories SET name = ?1, color = ?2 WHERE id = ?3",
            params![name, color, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> rusqlite::Result<()> {
        conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;
        Ok(())
    }
}
