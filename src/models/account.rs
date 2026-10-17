//! Model `Account` — padanan dari `models/Account.php`.

use rusqlite::{params, Connection, Row};

#[derive(Clone, Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub kind: String,
    /// Saldo awal akun (kolom `balance` pada tabel `accounts`).
    pub balance: f64,
    /// Kolom `created_at` pada tabel `accounts`.
    #[allow(dead_code)]
    pub created_at: String,
}

/// Ringkasan akun beserta total pemasukan/pengeluaran dan saldo saat ini.
#[derive(Clone, Debug)]
pub struct AccountSummary {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub initial_balance: f64,
    pub total_income: f64,
    pub total_expense: f64,
    pub current_balance: f64,
    /// Jumlah transaksi yang memakai akun ini (hasil `COUNT(t.id)`).
    #[allow(dead_code)]
    pub transaction_count: i64,
}

impl Account {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            kind: row.get("type")?,
            balance: row.get("balance")?,
            created_at: row
                .get::<_, Option<String>>("created_at")?
                .unwrap_or_default(),
        })
    }

    pub fn all(conn: &Connection) -> rusqlite::Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT * FROM accounts ORDER BY id")?;
        let rows = stmt.query_map([], Self::from_row)?;
        rows.collect()
    }

    pub fn get(conn: &Connection, id: i64) -> rusqlite::Result<Option<Self>> {
        let mut stmt = conn.prepare("SELECT * FROM accounts WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], Self::from_row)?;
        match rows.next() {
            Some(Ok(acc)) => Ok(Some(acc)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }


    pub fn create(conn: &Connection, name: &str, kind: &str, balance: f64) -> rusqlite::Result<i64> {
        conn.execute(
            "INSERT INTO accounts (name, type, balance) VALUES (?1, ?2, ?3)",
            params![name, kind, balance],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update(
        conn: &Connection,
        id: i64,
        name: &str,
        kind: &str,
        balance: f64,
    ) -> rusqlite::Result<()> {
        conn.execute(
            "UPDATE accounts SET name = ?1, type = ?2, balance = ?3 WHERE id = ?4",
            params![name, kind, balance, id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> rusqlite::Result<()> {
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Jumlah saldo awal dari seluruh akun (kolom `balance` pada `accounts`).
    pub fn initial_total(conn: &Connection) -> rusqlite::Result<f64> {
        conn.query_row("SELECT COALESCE(SUM(balance), 0) FROM accounts", [], |r| {
            r.get(0)
        })
    }

    /// Ringkasan tiap akun: saldo awal + pemasukan − pengeluaran.
    pub fn summary(conn: &Connection) -> rusqlite::Result<Vec<AccountSummary>> {
        let mut stmt = conn.prepare(
            r#"
            SELECT
                a.id,
                a.name,
                a.type,
                a.balance AS initial_balance,
                COALESCE(SUM(CASE WHEN t.type = 'income' THEN t.amount ELSE 0 END), 0) AS total_income,
                COALESCE(SUM(CASE WHEN t.type = 'expense' THEN t.amount ELSE 0 END), 0) AS total_expense,
                (a.balance + COALESCE(SUM(CASE WHEN t.type = 'income' THEN t.amount ELSE -t.amount END), 0)) AS current_balance,
                COUNT(t.id) AS transaction_count
            FROM accounts a
            LEFT JOIN transactions t ON a.id = t.account_id
            GROUP BY a.id, a.name, a.type, a.balance
            ORDER BY current_balance DESC
            "#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(AccountSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                kind: row.get(2)?,
                initial_balance: row.get(3)?,
                total_income: row.get(4)?,
                total_expense: row.get(5)?,
                current_balance: row.get(6)?,
                transaction_count: row.get(7)?,
            })
        })?;
        rows.collect()
    }
}
