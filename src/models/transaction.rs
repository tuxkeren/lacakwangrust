//! Model `Transaction` — padanan dari `models/Transaction.php`.

use rusqlite::{params, Connection, Row};

/// Satu baris transaksi lengkap dengan nama kategori & akun (untuk daftar).
#[derive(Clone, Debug)]
pub struct TransactionRow {
    pub id: i64,
    /// Kolom `category_id` (dipakai form edit lewat [`Transaction::get`]).
    #[allow(dead_code)]
    pub category_id: i64,
    /// Kolom `account_id`.
    #[allow(dead_code)]
    pub account_id: Option<i64>,
    pub amount: f64,
    pub kind: String,
    pub description: Option<String>,
    pub date: String,
    pub category_name: String,
    pub color: String,
    pub account_name: Option<String>,
}

/// Data transaksi tunggal (untuk form edit).
#[derive(Clone, Debug)]
pub struct TransactionDetail {
    pub id: i64,
    pub category_id: i64,
    pub account_id: Option<i64>,
    pub amount: f64,
    pub kind: String,
    pub description: Option<String>,
    pub date: String,
}

/// Total pemasukan & pengeluaran 30 hari terakhir.
#[derive(Clone, Copy, Debug, Default)]
pub struct Summary {
    pub income: f64,
    pub expense: f64,
}

/// Agregat pengeluaran/pemasukan per kategori.
#[derive(Clone, Debug)]
pub struct CategoryBreakdown {
    #[allow(dead_code)]
    pub id: i64,
    pub name: String,
    pub color: String,
    pub total: f64,
    #[allow(dead_code)]
    pub count: i64,
}

pub struct Transaction;

impl Transaction {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<TransactionRow> {
        Ok(TransactionRow {
            id: row.get(0)?,
            category_id: row.get(1)?,
            account_id: row.get(2)?,
            amount: row.get(3)?,
            kind: row.get(4)?,
            description: row.get(5)?,
            date: row.get(6)?,
            category_name: row.get(7)?,
            color: row.get(8)?,
            account_name: row.get(9)?,
        })
    }

    const SELECT_LIST: &'static str = r#"
        SELECT t.id, t.category_id, t.account_id, t.amount, t.type, t.description, t.date,
               c.name AS category_name, c.color AS color, a.name AS account_name
        FROM transactions t
        JOIN categories c ON t.category_id = c.id
        LEFT JOIN accounts a ON t.account_id = a.id
    "#;

    /// Daftar transaksi terbaru dengan paginasi.
    pub fn all(conn: &Connection, limit: i64, offset: i64) -> rusqlite::Result<Vec<TransactionRow>> {
        let sql = format!(
            "{} ORDER BY t.date DESC, t.created_at DESC LIMIT ?1 OFFSET ?2",
            Self::SELECT_LIST
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![limit, offset], Self::from_row)?;
        rows.collect()
    }

    /// Daftar transaksi dengan filter tipe.
    pub fn by_type(
        conn: &Connection,
        kind: &str,
        limit: i64,
        offset: i64,
    ) -> rusqlite::Result<Vec<TransactionRow>> {
        let sql = format!(
            "{} WHERE t.type = ?1 ORDER BY t.date DESC, t.created_at DESC LIMIT ?2 OFFSET ?3",
            Self::SELECT_LIST
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![kind, limit, offset], Self::from_row)?;
        rows.collect()
    }

    pub fn get(conn: &Connection, id: i64) -> rusqlite::Result<Option<TransactionDetail>> {
        let mut stmt = conn.prepare(
            r#"
            SELECT t.id, t.category_id, t.account_id, t.amount, t.type, t.description, t.date
            FROM transactions t
            WHERE t.id = ?1
            "#,
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(TransactionDetail {
                id: row.get(0)?,
                category_id: row.get(1)?,
                account_id: row.get(2)?,
                amount: row.get(3)?,
                kind: row.get(4)?,
                description: row.get(5)?,
                date: row.get(6)?,
            })
        })?;
        match rows.next() {
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create(
        conn: &Connection,
        category_id: i64,
        account_id: i64,
        amount: f64,
        kind: &str,
        description: Option<&str>,
        date: &str,
    ) -> rusqlite::Result<i64> {
        conn.execute(
            r#"
            INSERT INTO transactions (category_id, account_id, amount, type, description, date)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![category_id, account_id, amount, kind, description, date],
        )?;
        Ok(conn.last_insert_rowid())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        conn: &Connection,
        id: i64,
        category_id: i64,
        account_id: i64,
        amount: f64,
        description: Option<&str>,
        date: &str,
    ) -> rusqlite::Result<()> {
        conn.execute(
            r#"
            UPDATE transactions
            SET category_id = ?1, account_id = ?2, amount = ?3, description = ?4,
                date = ?5, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?6
            "#,
            params![category_id, account_id, amount, description, date, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> rusqlite::Result<()> {
        conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Ringkasan 30 hari terakhir (pemasukan & pengeluaran).
    pub fn summary(conn: &Connection) -> rusqlite::Result<Summary> {
        let mut stmt = conn.prepare(
            r#"
            SELECT type, SUM(amount) AS total
            FROM transactions
            WHERE date >= DATE('now', '-30 days')
            GROUP BY type
            "#,
        )?;
        let mut rows = stmt.query([])?;
        let mut summary = Summary::default();
        while let Some(row) = rows.next()? {
            let kind: String = row.get(0)?;
            let total: f64 = row.get::<_, Option<f64>>(1)?.unwrap_or(0.0);
            match kind.as_str() {
                "income" => summary.income = total,
                "expense" => summary.expense = total,
                _ => {}
            }
        }
        Ok(summary)
    }

    /// Agregat per kategori untuk tipe tertentu dalam `days` hari terakhir.
    pub fn category_breakdown(
        conn: &Connection,
        kind: &str,
        days: i64,
    ) -> rusqlite::Result<Vec<CategoryBreakdown>> {
        let mut stmt = conn.prepare(
            r#"
            SELECT c.id, c.name, c.color, SUM(t.amount) AS total, COUNT(t.id) AS count
            FROM transactions t
            JOIN categories c ON t.category_id = c.id
            WHERE t.type = ?1 AND t.date >= DATE('now', ?2 || ' days')
            GROUP BY c.id, c.name, c.color
            ORDER BY total DESC
            "#,
        )?;
        let offset = format!("-{days}");
        let rows = stmt.query_map(params![kind, offset], |row| {
            Ok(CategoryBreakdown {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                total: row.get(3)?,
                count: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn count(conn: &Connection) -> rusqlite::Result<i64> {
        conn.query_row("SELECT COUNT(*) FROM transactions", [], |r| r.get(0))
    }

    pub fn count_by_type(conn: &Connection, kind: &str) -> rusqlite::Result<i64> {
        conn.query_row(
            "SELECT COUNT(*) FROM transactions WHERE type = ?1",
            params![kind],
            |r| r.get(0),
        )
    }
}
