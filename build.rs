//! Skrip build: menyisipkan ikon dan metadata ke file `.exe` Windows.
//!
//! Hanya melakukan sesuatu bila **target**-nya Windows (termasuk saat
//! dikompilasi silang dari Linux). Untuk target lain, skrip ini langsung
//! selesai tanpa efek apa pun.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");

    // `CARGO_CFG_TARGET_OS` adalah OS target, bukan OS mesin yang dipakai
    // membangun — jadi tetap benar saat cross-compile.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("ProductName", "LacakWangRust");
    res.set(
        "FileDescription",
        "LacakWangRust - Pencatatan Keuangan Sederhana",
    );
    res.set("OriginalFilename", "lacakwangrust.exe");
    // Kredit pembuat juga ikut tertulis di properti berkas `.exe`
    // (tab "Details" di Windows), bukan hanya di dalam aplikasi.
    res.set("CompanyName", "Athailah");
    res.set("LegalCopyright", "\u{00A9} 2026 Athailah - MIT License");
    res.set(
        "Comments",
        "Dibuat oleh Athailah (tuxkeren) - https://github.com/tuxkeren/lacakwangrust",
    );
    res.set("FileVersion", "1.0.0.0");
    res.set("ProductVersion", "1.0.0.0");

    // Kalau penyusun resource tidak tersedia, jangan gagalkan build —
    // aplikasi tetap jalan, hanya tanpa ikon di dalam .exe.
    if let Err(e) = res.compile() {
        println!("cargo:warning=gagal menyusun resource Windows: {e}");
    }
}
