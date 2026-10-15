//! Controller dashboard — padanan dari `controllers/DashboardController.php`.

use rusqlite::Connection;

use crate::models::{Account, Transaction, TransactionRow};
use crate::models::transaction::{CategoryBreakdown, Summary};

/// Seluruh data yang dibutuhkan halaman Dashboard.
#[derive(Clone, Debug, Default)]
pub struct DashboardData {
    pub summary: Summary,
    /// Total saldo gabungan seluruh akun (saldo awal + pemasukan − pengeluaran).
    pub balance: f64,
    pub income_breakdown: Vec<CategoryBreakdown>,
    pub expense_breakdown: Vec<CategoryBreakdown>,
    pub recent_transactions: Vec<TransactionRow>,
}

pub struct DashboardController;

impl DashboardController {
    pub fn index(conn: &Connection) -> rusqlite::Result<DashboardData> {
        let summary = Transaction::summary(conn)?;
        let income_breakdown = Transaction::category_breakdown(conn, "income", 30)?;
        let expense_breakdown = Transaction::category_breakdown(conn, "expense", 30)?;
        let recent_transactions = Transaction::all(conn, 5, 0)?;

        let balance = Account::summary(conn)?
            .iter()
            .map(|a| a.current_balance)
            .sum();

        Ok(DashboardData {
            summary,
            balance,
            income_breakdown,
            expense_breakdown,
            recent_transactions,
        })
    }
}
