//! Konfigurasi aplikasi.
//!
//! Padanan dari `config/config.php` pada versi PHP.

use std::path::PathBuf;

/// Konfigurasi global aplikasi.
#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub currency_symbol: String,
    pub currency_code: String,
    /// Jumlah baris per halaman pada daftar transaksi.
    pub per_page: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "LacakWangRust".to_string(),
            version: "1.0.0".to_string(),
            currency_symbol: "Rp".to_string(),
            currency_code: "IDR".to_string(),
            per_page: 10,
        }
    }
}

/// Lokasi file database SQLite.
///
/// Bisa dioverride lewat environment variable `LACAKWANGRUST_DB`,
/// sehingga aplikasi ini bahkan bisa membaca database `pelacakduit.db`
/// milik versi PHP bila diinginkan.
pub fn resolve_db_path() -> PathBuf {
    if let Ok(path) = std::env::var("LACAKWANGRUST_DB") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }
    PathBuf::from("database").join("lacakwangrust.db")
}
