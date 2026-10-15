//! Controller transaksi — padanan dari `controllers/TransactionController.php`.

use rusqlite::Connection;

use crate::models::{Account, Category, Transaction, TransactionRow};

/// Filter daftar transaksi (tombol "Semua / Pemasukan / Pengeluaran").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionFilter {
    All,
    Income,
    Expense,
}

impl TransactionFilter {
    pub fn sql_type(self) -> Option<&'static str> {
        match self {
            Self::All => None,
            Self::Income => Some("income"),
            Self::Expense => Some("expense"),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "Semua",
            Self::Income => "Pemasukan",
            Self::Expense => "Pengeluaran",
        }
    }
}

/// Data yang dibutuhkan halaman daftar transaksi.
#[derive(Clone, Debug)]
pub struct TransactionListData {
    pub transactions: Vec<TransactionRow>,
    pub current_page: usize,
    pub total_pages: usize,
    pub per_page: usize,
    pub total_count: i64,
}

pub struct TransactionController;

impl TransactionController {
    pub fn index(
        conn: &Connection,
        filter: TransactionFilter,
        page: usize,
        per_page: usize,
    ) -> rusqlite::Result<TransactionListData> {
        let per_page = per_page.max(1);
        let page = page.max(1);
        let offset = ((page - 1) * per_page) as i64;

        let total_count = match filter.sql_type() {
            Some(kind) => Transaction::count_by_type(conn, kind)?,
            None => Transaction::count(conn)?,
        };

        let transactions = match filter.sql_type() {
            Some(kind) => Transaction::by_type(conn, kind, per_page as i64, offset)?,
            None => Transaction::all(conn, per_page as i64, offset)?,
        };

        let total_pages = ((total_count as f64 / per_page as f64).ceil() as usize).max(1);
        let current_page = page.min(total_pages);

        Ok(TransactionListData {
            transactions,
            current_page,
            total_pages,
            per_page,
            total_count,
        })
    }

    /// Data untuk form "Tambah Transaksi".
    pub fn create_data(conn: &Connection) -> rusqlite::Result<(Vec<Category>, Vec<Account>)> {
        Ok((Category::all(conn)?, Account::all(conn)?))
    }

    /// Data untuk form "Edit Transaksi".
    pub fn edit_data(
        conn: &Connection,
        id: i64,
    ) -> rusqlite::Result<Option<(crate::models::TransactionDetail, Vec<Category>, Vec<Account>)>> {
        match Transaction::get(conn, id)? {
            Some(detail) => Ok(Some((
                detail,
                Category::all(conn)?,
                Account::all(conn)?,
            ))),
            None => Ok(None),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn store(
        conn: &Connection,
        category_id: i64,
        account_id: i64,
        amount: f64,
        kind: &str,
        description: Option<&str>,
        date: &str,
    ) -> Result<i64, String> {
        if category_id <= 0 {
            return Err("Kategori wajib dipilih".into());
        }
        if amount <= 0.0 {
            return Err("Jumlah harus lebih besar dari 0".into());
        }
        if kind != "income" && kind != "expense" {
            return Err("Tipe transaksi tidak valid".into());
        }
        if !crate::util::is_valid_iso_date(date) {
            return Err("Tanggal tidak valid (format YYYY-MM-DD)".into());
        }

        Transaction::create(
            conn,
            category_id,
            account_id,
            amount,
            kind,
            description,
            date,
        )
        .map_err(|e| e.to_string())
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
    ) -> Result<(), String> {
        if id <= 0 {
            return Err("ID transaksi tidak valid".into());
        }
        if category_id <= 0 {
            return Err("Kategori wajib dipilih".into());
        }
        if amount <= 0.0 {
            return Err("Jumlah harus lebih besar dari 0".into());
        }
        if !crate::util::is_valid_iso_date(date) {
            return Err("Tanggal tidak valid (format YYYY-MM-DD)".into());
        }

        Transaction::update(
            conn,
            id,
            category_id,
            account_id,
            amount,
            description,
            date,
        )
        .map_err(|e| e.to_string())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), String> {
        if id <= 0 {
            return Err("ID transaksi tidak valid".into());
        }
        Transaction::delete(conn, id).map_err(|e| e.to_string())
    }
}
