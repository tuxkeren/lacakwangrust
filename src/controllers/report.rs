//! Controller laporan — merangkai data laporan keuangan dari database.
//!
//! Padanan `controllers/ReportController.php` pada versi PHP (bila ada);
//! di sini laporan dipakai untuk mencetak PDF dua kolom pemasukan dan
//! pengeluaran.

use rusqlite::Connection;

use crate::models::{Account, ReportData, ReportPeriod, Transaction};

pub struct ReportController;

impl ReportController {
    /// Menyusun data laporan untuk periode tertentu.
    ///
    /// Transaksi dipisah menjadi dua daftar (pemasukan & pengeluaran) dan
    /// `opening_balance` dihitung dari saldo awal seluruh akun ditambah
    /// selisih transaksi sebelum periode dimulai.
    pub fn build(conn: &Connection, period: ReportPeriod) -> rusqlite::Result<ReportData> {
        let (from, to) = period.range();
        let from_iso = from.to_string();
        let to_iso = to.to_string();

        let semua = Transaction::in_range(conn, &from_iso, &to_iso)?;
        let mut income = Vec::new();
        let mut expense = Vec::new();
        for t in semua {
            if t.kind == "income" {
                income.push(t);
            } else {
                expense.push(t);
            }
        }

        let total_income: f64 = income.iter().map(|t| t.amount).sum();
        let total_expense: f64 = expense.iter().map(|t| t.amount).sum();

        let opening_balance =
            Account::initial_total(conn)? + Transaction::net_before(conn, &from_iso)?;

        Ok(ReportData {
            period,
            from,
            to,
            income,
            expense,
            total_income,
            total_expense,
            opening_balance,
        })
    }
}
