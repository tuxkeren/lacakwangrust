//! Controller akun/dompet — padanan dari `controllers/AccountController.php`.

use rusqlite::Connection;

use crate::models::{Account, AccountSummary};

/// Data halaman Manajemen Akun.
#[derive(Clone, Debug, Default)]
pub struct AccountListData {
    pub accounts: Vec<AccountSummary>,
    pub total_income: f64,
    pub total_expense: f64,
    pub total_balance: f64,
}

pub struct AccountController;

impl AccountController {
    pub fn index(conn: &Connection) -> rusqlite::Result<AccountListData> {
        let accounts = Account::summary(conn)?;
        let total_income = accounts.iter().map(|a| a.total_income).sum();
        let total_expense = accounts.iter().map(|a| a.total_expense).sum();
        let total_balance = accounts.iter().map(|a| a.current_balance).sum();

        Ok(AccountListData {
            accounts,
            total_income,
            total_expense,
            total_balance,
        })
    }

    /// Ambil satu akun apa adanya dari database (dipakai untuk mengisi form edit).
    pub fn detail(conn: &Connection, id: i64) -> rusqlite::Result<Option<Account>> {
        Account::get(conn, id)
    }

    /// Simpan akun baru (id `None`) atau perbarui akun yang ada.
    pub fn save(
        conn: &Connection,
        id: Option<i64>,
        name: &str,
        kind: &str,
        balance: f64,
    ) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("Nama akun diperlukan".into());
        }
        if !matches!(kind, "cash" | "bank" | "ewallet" | "other") {
            return Err("Tipe akun tidak valid".into());
        }

        match id {
            Some(id) if id > 0 => {
                Account::update(conn, id, name, kind, balance).map_err(|e| e.to_string())
            }
            _ => Account::create(conn, name, kind, balance)
                .map(|_| ())
                .map_err(|e| e.to_string()),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), String> {
        Account::delete(conn, id).map_err(|e| e.to_string())
    }
}
