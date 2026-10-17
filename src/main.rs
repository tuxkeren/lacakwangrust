//! LacakWangRust — Pencatatan uang masuk & uang keluar sederhana.
//!
//! Versi native (desktop) dari aplikasi PHP OOP "PelacakDuit", dibangun
//! dengan Rust + egui. Struktur kode tetap mengikuti pola MVC:
//! `models` (data), `controllers` (logika), `ui` (tampilan).

// Di Windows, aplikasi dijalankan sebagai aplikasi GUI murni supaya tidak
// memunculkan jendela konsol hitam saat di-double-click. (Tanpa efek di Linux.)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod controllers;
mod db;
mod models;
mod pdf;
mod ui;
mod util;

use eframe::egui;

/// Ikon jendela (RGBA mentah 64x64, alpha tidak ter-premultiply).
/// Dibuat oleh `tools/make_icon.py`; ikon `.ico` untuk .exe Windows juga
/// berasal dari skrip yang sama.
const ICON_RGBA: &[u8] = include_bytes!("../assets/icon-64.rgba");
const ICON_SIZE: u32 = 64;

/// Menampilkan pesan error fatal lalu keluar.
///
/// Di Windows dipakai kotak dialog, sebab aplikasi GUI tidak memiliki jendela
/// konsol tempat `eprintln!` bisa terlihat.
fn fail(message: &str) -> ! {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;

        #[link(name = "user32")]
        extern "system" {
            fn MessageBoxW(
                hwnd: *mut core::ffi::c_void,
                text: *const u16,
                caption: *const u16,
                u_type: u32,
            ) -> i32;
        }

        let wide = |s: &str| -> Vec<u16> {
            std::ffi::OsStr::new(s)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect()
        };
        let text = wide(message);
        let caption = wide("Lacakuang");
        // MB_OK | MB_ICONERROR | MB_SETFOREGROUND
        let flags = 0x0 | 0x10 | 0x10000;
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                caption.as_ptr(),
                flags,
            );
        }
    }

    #[cfg(not(windows))]
    eprintln!("{message}");

    std::process::exit(1);
}

fn main() -> eframe::Result<()> {
    let app_config = config::Config::default();
    let db_path = config::resolve_db_path();

    util::set_currency_symbol(&app_config.currency_symbol);

    let database = match db::Database::open(&db_path) {
        Ok(db) => db,
        Err(e) => fail(&format!(
            "Gagal membuka database di {}:\n{e}",
            db_path.display()
        )),
    };

    if let Err(e) = db::schema::init(database.conn()) {
        fail(&format!("Gagal menyiapkan skema database:\n{e}"));
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 820.0])
            .with_min_inner_size([900.0, 620.0])
            .with_title("Lacakuang")
            .with_icon(egui::IconData {
                rgba: ICON_RGBA.to_vec(),
                width: ICON_SIZE,
                height: ICON_SIZE,
            }),
        ..Default::default()
    };

    match eframe::run_native(
        "Lacakuang",
        native_options,
        Box::new(move |cc| Ok(Box::new(ui::App::new(cc, app_config, database)))),
    ) {
        Ok(()) => Ok(()),
        Err(e) => fail(&format!("Gagal menjalankan aplikasi:\n{e}")),
    }
}
