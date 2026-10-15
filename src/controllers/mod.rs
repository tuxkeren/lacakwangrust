//! Lapisan controller (padanan folder `controllers/` pada versi PHP).
//!
//! Setiap controller menerima referensi koneksi database dan mengembalikan
//! data siap-pakai untuk lapisan UI.

pub mod account;
pub mod category;
pub mod dashboard;
pub mod transaction;

pub use account::AccountController;
pub use category::CategoryController;
pub use dashboard::DashboardController;
pub use transaction::{TransactionController, TransactionFilter};
