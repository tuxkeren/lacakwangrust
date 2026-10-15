//! Pembuatan skema dan data awal.
//!
//! Padanan dari `config/Schema.php` pada versi PHP — struktur tabel,
//! akun default, dan kategori default dibuat identik.

use rusqlite::{params, Connection};

/// Membuat seluruh tabel bila belum ada, lalu mengisi data default.
pub fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        VARCHAR(100) NOT NULL,
            type        VARCHAR(10)  NOT NULL CHECK(type IN ('income','expense')),
            color       VARCHAR(7)   DEFAULT '#3B82F6',
            created_at  DATETIME     DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        VARCHAR(100) NOT NULL,
            type        VARCHAR(50)  DEFAULT 'bank',
            balance     DECIMAL(15,2) DEFAULT 0,
            created_at  DATETIME     DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            category_id INTEGER      NOT NULL,
            account_id  INTEGER      DEFAULT 1,
            amount      DECIMAL(15,2) NOT NULL,
            type        VARCHAR(10)  NOT NULL CHECK(type IN ('income','expense')),
            description TEXT,
            date        DATE         NOT NULL,
            created_at  DATETIME     DEFAULT CURRENT_TIMESTAMP,
            updated_at  DATETIME     DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE SET NULL
        );
        "#,
    )?;

    // Auto-migrasi: tambahkan kolom account_id bila database lama belum punya.
    let mut has_account_id = false;
    {
        let mut stmt = conn.prepare("PRAGMA table_info(transactions)")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let name: String = row.get(1)?;
            if name == "account_id" {
                has_account_id = true;
                break;
            }
        }
    }
    if !has_account_id {
        conn.execute_batch("ALTER TABLE transactions ADD COLUMN account_id INTEGER DEFAULT 1")?;
    }

    // Data akun default (hanya bila tabel masih kosong).
    let account_count: i64 = conn.query_row("SELECT COUNT(*) FROM accounts", [], |r| r.get(0))?;
    if account_count == 0 {
        let default_accounts = [
            ("Tunai / Cash", "cash", 0.0_f64),
            ("Bank Mandiri", "bank", 0.0),
            ("BSI (Bank Syariah Indonesia)", "bank", 0.0),
            ("E-Wallet (GoPay/OVO)", "ewallet", 0.0),
        ];
        let mut stmt =
            conn.prepare("INSERT INTO accounts (name, type, balance) VALUES (?1, ?2, ?3)")?;
        for (name, kind, balance) in default_accounts {
            stmt.execute(params![name, kind, balance])?;
        }
    }

    // Data kategori default (hanya bila tabel masih kosong).
    let category_count: i64 = conn.query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0))?;
    if category_count == 0 {
        let default_categories = [
            ("Gaji", "income", "#10B981"),
            ("Penjualan", "income", "#3B82F6"),
            ("Bonus", "income", "#8B5CF6"),
            ("Makanan", "expense", "#EF4444"),
            ("Transportasi", "expense", "#F59E0B"),
            ("Belanja", "expense", "#EC4899"),
            ("Tagihan", "expense", "#6366F1"),
            ("Lainnya", "expense", "#6B7280"),
        ];
        let mut stmt = conn.prepare("INSERT INTO categories (name, type, color) VALUES (?1, ?2, ?3)")?;
        for (name, kind, color) in default_categories {
            stmt.execute(params![name, kind, color])?;
        }
    }

    Ok(())
}
