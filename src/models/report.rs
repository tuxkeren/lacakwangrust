//! Model laporan keuangan (periode + data yang dipakai untuk mencetak PDF).
//!
//! Model ini tidak punya tabel sendiri: datanya disusun dari tabel
//! `transactions` dan `accounts` oleh
//! [`ReportController`](crate::controllers::ReportController).

use chrono::{Datelike, NaiveDate};

use super::transaction::TransactionRow;

/// Rentang tanggal paling awal/akhir yang dipakai untuk periode "semua data".
const MIN_DATE: (i32, u32, u32) = (1970, 1, 1);
const MAX_DATE: (i32, u32, u32) = (9999, 12, 31);

/// Periode yang dicakup laporan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportPeriod {
    /// Seluruh transaksi yang ada.
    All,
    /// Bulan berjalan menurut waktu lokal.
    ThisMonth,
    /// Tahun berjalan menurut waktu lokal.
    ThisYear,
}

impl ReportPeriod {
    /// Label singkat untuk tombol pilihan.
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "Semua Data",
            Self::ThisMonth => "Bulan Ini",
            Self::ThisYear => "Tahun Ini",
        }
    }

    /// Batas `[awal, akhir]` (inklusif) untuk query database.
    pub fn range(self) -> (NaiveDate, NaiveDate) {
        let today = chrono::Local::now().date_naive();
        let first = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).expect("tanggal valid");

        match self {
            Self::All => (first(MIN_DATE.0, MIN_DATE.1, MIN_DATE.2), first(MAX_DATE.0, MAX_DATE.1, MAX_DATE.2)),
            Self::ThisMonth => {
                let start = first(today.year(), today.month(), 1);
                let end = match (today.month(), today.year()) {
                    (12, y) => first(y + 1, 1, 1),
                    (m, y) => first(y, m + 1, 1),
                }
                .pred_opt()
                .expect("akhir bulan valid");
                (start, end)
            }
            Self::ThisYear => (
                first(today.year(), 1, 1),
                first(today.year(), 12, 31),
            ),
        }
    }
}

/// Seluruh data yang dibutuhkan untuk mencetak laporan PDF.
#[derive(Clone, Debug)]
pub struct ReportData {
    pub period: ReportPeriod,
    /// Tanggal awal periode (format ISO).
    pub from: NaiveDate,
    /// Tanggal akhir periode (format ISO).
    pub to: NaiveDate,
    /// Transaksi pemasukan dalam periode, urut dari yang terlama.
    pub income: Vec<TransactionRow>,
    /// Transaksi pengeluaran dalam periode, urut dari yang terlama.
    pub expense: Vec<TransactionRow>,
    pub total_income: f64,
    pub total_expense: f64,
    /// Saldo seluruh akun tepat sebelum periode dimulai.
    pub opening_balance: f64,
}

impl ReportData {
    /// Selisih pemasukan dan pengeluaran dalam periode.
    pub fn net(&self) -> f64 {
        self.total_income - self.total_expense
    }

    /// Sisa saldo pada akhir periode (saldo awal + pemasukan − pengeluaran).
    pub fn closing_balance(&self) -> f64 {
        self.opening_balance + self.net()
    }
}
