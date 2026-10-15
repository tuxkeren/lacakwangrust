//! Fungsi bantu: format mata uang, tanggal, dan konversi warna.
//!
//! Padanan dari helper `formatRupiah()` yang dipakai berulang di view PHP.

use eframe::egui::Color32;
use std::sync::OnceLock;

/// Simbol mata uang yang dipakai `format_rupiah()`.
///
/// Diisi sekali saat aplikasi mulai lewat [`set_currency_symbol`], sehingga
/// nilainya tetap bisa dikonfigurasi (lihat `config::Config`).
static CURRENCY_SYMBOL: OnceLock<String> = OnceLock::new();

/// Menetapkan simbol mata uang, mis. `"Rp"`. Hanya berpengaruh bila dipanggil
/// sebelum tampilan pertama digambar.
pub fn set_currency_symbol(symbol: &str) {
    let _ = CURRENCY_SYMBOL.set(symbol.to_string());
}

/// Simbol mata uang aktif (default `"Rp"`).
fn currency_symbol() -> &'static str {
    CURRENCY_SYMBOL.get().map(String::as_str).unwrap_or("Rp")
}

/// Memformat angka dengan pemisah ribuan titik, dibulatkan ke bilangan bulat.
///
/// `1234567.4` -> `"1.234.567"`
pub fn format_number(amount: f64) -> String {
    let negative = amount < 0.0;
    let value = amount.abs().round() as i128;
    let digits = value.to_string();
    let bytes = digits.as_bytes();

    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(*b as char);
    }

    if negative {
        format!("-{out}")
    } else {
        out
    }
}

/// `1234567` -> `"Rp 1.234.567"` (meniru `number_format($amount, 0, ',', '.')`).
pub fn format_rupiah(amount: f64) -> String {
    format!("{} {}", currency_symbol(), format_number(amount))
}

/// Tanggal hari ini dalam format `YYYY-MM-DD`.
pub fn today_iso() -> String {
    chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string()
}

/// Mengubah `2026-10-15` menjadi `15/10/2026`.
pub fn format_date_id(date: &str) -> String {
    match chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(d) => d.format("%d/%m/%Y").to_string(),
        Err(_) => date.to_string(),
    }
}

/// Validasi sederhana format tanggal `YYYY-MM-DD`.
pub fn is_valid_iso_date(date: &str) -> bool {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

/// Mengubah kode warna heksadesimal (`#RRGGBB`) menjadi [`Color32`].
pub fn hex_to_color(hex: &str) -> Color32 {
    let h = hex.trim().trim_start_matches('#');
    if h.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&h[0..2], 16),
            u8::from_str_radix(&h[2..4], 16),
            u8::from_str_radix(&h[4..6], 16),
        ) {
            return Color32::from_rgb(r, g, b);
        }
    }
    crate::ui::theme::PRIMARY
}

/// Warna penanda tipe akun (sama dengan versi PHP).
pub fn account_type_color(kind: &str) -> Color32 {
    use crate::ui::theme as t;
    match kind {
        "cash" => t::GREEN_500,
        "bank" => t::BLUE_500,
        "ewallet" => t::PURPLE_500,
        _ => t::GRAY_500,
    }
}

/// Label tipe akun berbahasa Indonesia (padanan `ucfirst($type)`).
pub fn account_type_label(kind: &str) -> &'static str {
    match kind {
        "cash" => "Tunai",
        "bank" => "Bank",
        "ewallet" => "E-Wallet",
        _ => "Lainnya",
    }
}

/// Ikon emoji untuk tiap tipe akun (dipakai di form).
pub fn account_type_icon(kind: &str) -> &'static str {
    match kind {
        "cash" => "\u{1F4B5}",    // 💵
        "bank" => "\u{1F3E6}",    // 🏦
        "ewallet" => "\u{1F4F1}", // 📱
        _ => "\u{1F4E6}",         // 📦
    }
}

/// Label tipe transaksi.
pub fn transaction_type_label(kind: &str) -> &'static str {
    if kind == "income" {
        "Masuk"
    } else {
        "Keluar"
    }
}

/// Memotong teks yang terlalu panjang dan menambahkan elipsis.
pub fn truncate(text: &str, max_chars: usize) -> String {
    let count = text.chars().count();
    if count <= max_chars {
        return text.to_string();
    }
    let kept: String = text.chars().take(max_chars.saturating_sub(1)).collect();
    format!("{kept}\u{2026}")
}

/// Mengurai angka dari input pengguna.
///
/// Menerima `"50000"`, `"50.000"`, `"1.500.000"`, maupun `"50000,50"`.
pub fn parse_amount(input: &str) -> Option<f64> {
    let mut cleaned: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != 'R' && *c != 'p')
        .collect();

    if cleaned.is_empty() {
        return None;
    }

    // Normalisasi: koma dianggap desimal.
    cleaned = cleaned.replace(',', ".");

    let dots = cleaned.matches('.').count();
    if dots > 1 {
        // Banyak titik -> semua dianggap pemisah ribuan kecuali yang terakhir.
        if let Some(idx) = cleaned.rfind('.') {
            let (head, tail) = cleaned.split_at(idx);
            cleaned = format!("{}{}", head.replace('.', ""), tail);
        }
    } else if dots == 1 {
        // Satu titik: kalau tepat 3 digit di belakangnya, anggap pemisah ribuan.
        if let Some(idx) = cleaned.rfind('.') {
            let after = &cleaned[idx + 1..];
            if after.len() == 3 && after.chars().all(|c| c.is_ascii_digit()) {
                cleaned = cleaned.replace('.', "");
            }
        }
    }

    cleaned.parse::<f64>().ok()
}
