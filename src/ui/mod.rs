//! Lapisan tampilan (UI) berbasis egui.
//!
//! Padanan folder `views/` pada versi PHP, tetapi dirender sebagai
//! aplikasi desktop native — bukan halaman web.

pub mod accounts;
pub mod app;
pub mod categories;
pub mod dashboard;
pub mod theme;
pub mod transactions;
pub mod widgets;

pub use app::App;
