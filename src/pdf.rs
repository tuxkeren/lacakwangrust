//! Penulis berkas PDF untuk laporan keuangan dua kolom.
//!
//! Laporan disusun di atas kertas A4 dengan dua kolom berdampingan:
//! **Pemasukan** di kiri dan **Pengeluaran** di kanan. Tiap baris memuat
//! tanggal, deskripsi, dan jumlah. Di bagian akhir ditampilkan ringkasan
//! beserta **sisa saldo** pada akhir periode.
//!
//! Berkas PDF ditulis langsung tanpa dependensi mesin render: halaman,
//! garis, kotak, dan teks memakai font bawaan PDF (Helvetica) yang sudah
//! tersedia di semua pembaca PDF.

use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use printpdf::path::PaintMode;
use printpdf::*;

use crate::models::transaction::TransactionRow;
use crate::models::ReportData;
use crate::util;

// ---- Ukuran halaman & tata letak (satuan milimeter) ----
const PAGE_W: f32 = 210.0; // A4
const PAGE_H: f32 = 297.0;
const MARGIN_X: f32 = 14.0;
/// Batas bawah area isi; di bawah ini hanya dipakai untuk catatan kaki.
const BOTTOM_Y: f32 = 22.0;
/// Lebar satu kolom laporan.
const COL_W: f32 = 88.0;
/// Jarak antara kolom pemasukan dan pengeluaran.
const COL_GAP: f32 = 6.0;
/// Tinggi satu baris tabel.
const ROW_H: f32 = 5.5;
/// Tinggi pita judul kolom.
const BAND_H: f32 = 7.0;
/// Jumlah maksimum karakter deskripsi di dalam tabel.
const DESC_MAX: usize = 30;

/// Warna teks & garis (0.0–1.0).
type Ink = (f32, f32, f32);
const INK: Ink = (0.11, 0.15, 0.22);
const INK_SOFT: Ink = (0.38, 0.43, 0.50);
const INK_FAINT: Ink = (0.55, 0.60, 0.66);
const INCOME_INK: Ink = (0.02, 0.40, 0.30);
const EXPENSE_INK: Ink = (0.72, 0.13, 0.13);
const RULE: Ink = (0.84, 0.86, 0.89);
const INCOME_BAND: Ink = (0.89, 0.98, 0.94);
const EXPENSE_BAND: Ink = (1.0, 0.93, 0.93);
const SUMMARY_BG: Ink = (0.95, 0.97, 0.99);

/// Identitas aplikasi yang dituliskan pada kop laporan.
pub struct ReportMeta {
    pub app_name: String,
    pub version: String,
    pub currency_code: String,
    pub author: String,
}

/// Nama berkas default untuk laporan PDF.
pub fn default_report_path(dir: &Path) -> PathBuf {
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    dir.join(format!("laporan-{stamp}.pdf"))
}

/// Menulis laporan `data` ke berkas PDF di `path`.
pub fn write_laporan(path: &Path, data: &ReportData, meta: &ReportMeta) -> Result<(), String> {
    if data.income.is_empty() && data.expense.is_empty() {
        return Err("Tidak ada transaksi pada periode ini, laporan tidak dibuat.".to_string());
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat folder {}: {e}", parent.display()))?;
        }
    }

    let (doc, page, layer) = PdfDocument::new("Laporan Keuangan", Mm(PAGE_W), Mm(PAGE_H), "Laporan");
    let doc = doc
        .with_author(meta.author.clone())
        .with_creator(format!("{} v{}", meta.app_name, meta.version))
        .with_subject("Laporan pemasukan dan pengeluaran");

    let regular = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Gagal menyiapkan font PDF: {e}"))?;
    let bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Gagal menyiapkan font PDF: {e}"))?;

    let mut w = Writer {
        doc: &doc,
        regular,
        bold,
        pages: vec![(page, layer)],
        layer: doc.get_page(page).get_layer(layer),
    };

    let left_x = MARGIN_X;
    let right_x = MARGIN_X + COL_W + COL_GAP;
    let content_right = PAGE_W - MARGIN_X;
    let cetak = chrono::Local::now().format("%d/%m/%Y %H:%M").to_string();
    let periode = periode_line(data);

    // ---------------- Kop halaman pertama ----------------
    let mut y = PAGE_H - 18.0;
    w.text("Laporan Keuangan", 17.0, MARGIN_X, y, true, INK);
    w.text_right(&format!("Dicetak: {cetak}"), 8.5, content_right, y, false, INK_FAINT);
    y -= 6.0;
    w.text(
        &format!(
            "{} v{} \u{00B7} Mata uang {} \u{00B7} Pemasukan & Pengeluaran",
            meta.app_name, meta.version, meta.currency_code
        ),
        9.0,
        MARGIN_X,
        y,
        false,
        INK_SOFT,
    );
    y -= 5.0;
    w.text(&format!("Periode: {periode}"), 9.0, MARGIN_X, y, false, INK_SOFT);
    y -= 5.0;
    w.hline(MARGIN_X, content_right, y, 0.9, RULE);

    // ---------------- Judul kedua kolom ----------------
    let mut y = w.column_heads(261.0, data, left_x, right_x);

    // ---------------- Isi tabel ----------------
    let baris = data.income.len().max(data.expense.len());
    for i in 0..baris {
        if y - ROW_H < BOTTOM_Y {
            y = w.continuation_page(&periode, data, left_x, right_x);
        }
        w.table_row(y, data.income.get(i), left_x, INCOME_INK);
        w.table_row(y, data.expense.get(i), right_x, EXPENSE_INK);
        y -= ROW_H;
    }

    // ---------------- Ringkasan & sisa saldo ----------------
    let ringkasan_dibutuhkan = 54.0;
    if y - ringkasan_dibutuhkan < BOTTOM_Y {
        y = w.continuation_page(&periode, data, left_x, right_x);
    }
    y -= 8.0;
    w.summary(y, data, left_x, content_right);

    // ---------------- Catatan kaki & nomor halaman ----------------
    w.footers(meta);

    // `w` meminjam `doc`; lepaskan dulu supaya `doc` bisa dipindahkan ke `save()`.
    drop(w);

    let file = File::create(path).map_err(|e| format!("Gagal membuat berkas PDF: {e}"))?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| format!("Gagal menulis berkas PDF: {e}"))?;
    Ok(())
}

/// Baris keterangan periode pada kop laporan.
fn periode_line(data: &ReportData) -> String {
    match data.period {
        crate::models::ReportPeriod::All => "Seluruh data yang tercatat".to_string(),
        _ => format!(
            "{} \u{00B7} {} s.d. {}",
            data.period.label(),
            util::format_date_id(&data.from.to_string()),
            util::format_date_id(&data.to.to_string())
        ),
    }
}

/// Pembantu penulisan ke satu lapisan PDF (satu halaman satu lapisan).
struct Writer<'a> {
    doc: &'a PdfDocumentReference,
    regular: IndirectFontRef,
    bold: IndirectFontRef,
    layer: PdfLayerReference,
    /// Daftar `(halaman, lapisan)` yang sudah dibuat, untuk mencetak catatan kaki.
    pages: Vec<(PdfPageIndex, PdfLayerIndex)>,
}

impl<'a> Writer<'a> {
    fn color(ink: Ink) -> Color {
        Color::Rgb(Rgb::new(ink.0, ink.1, ink.2, None))
    }

    /// Menulis teks dengan titik acuan kiri-bawah (baseline) pada `y`.
    fn text(&self, isi: &str, size: f32, x: f32, y: f32, bold: bool, ink: Ink) {
        self.layer.set_fill_color(Self::color(ink));
        let font = if bold { &self.bold } else { &self.regular };
        self.layer
            .use_text(winansi(isi), size, Mm(x), Mm(y), font);
    }

    /// Menulis teks dengan sisi kanan sejajar pada `right`.
    fn text_right(&self, isi: &str, size: f32, right: f32, y: f32, bold: bool, ink: Ink) {
        let bersih = winansi(isi);
        let x = right - text_width_mm(&bersih, size);
        self.text(&bersih, size, x, y, bold, ink);
    }

    fn hline(&self, x0: f32, x1: f32, y: f32, thickness: f32, ink: Ink) {
        self.layer.set_outline_color(Self::color(ink));
        self.layer.set_outline_thickness(thickness);
        self.layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x0), Mm(y)), false),
                (Point::new(Mm(x1), Mm(y)), false),
            ],
            is_closed: false,
        });
    }

    fn rect_fill(&self, x0: f32, y0: f32, x1: f32, y1: f32, ink: Ink) {
        self.layer.set_fill_color(Self::color(ink));
        self.layer
            .add_rect(Rect::new(Mm(x0), Mm(y0), Mm(x1), Mm(y1)).with_mode(PaintMode::Fill));
    }

    fn rect_stroke(&self, x0: f32, y0: f32, x1: f32, y1: f32, ink: Ink, thickness: f32) {
        self.layer.set_outline_color(Self::color(ink));
        self.layer.set_outline_thickness(thickness);
        self.layer
            .add_rect(Rect::new(Mm(x0), Mm(y0), Mm(x1), Mm(y1)).with_mode(PaintMode::Stroke));
    }

    /// Membuat halaman baru dan menjadikannya lapisan aktif.
    fn add_page(&mut self) {
        let (page, layer) = self.doc.add_page(Mm(PAGE_W), Mm(PAGE_H), "Laporan");
        self.pages.push((page, layer));
        self.layer = self.doc.get_page(page).get_layer(layer);
    }

    /// Pita judul + baris judul kolom; mengembalikan `y` baseline baris pertama.
    fn column_heads(
        &self,
        y_top: f32,
        data: &ReportData,
        left_x: f32,
        right_x: f32,
    ) -> f32 {
        self.column_head(
            "PEMASUKAN",
            data.income.len(),
            left_x,
            INCOME_INK,
            INCOME_BAND,
            y_top,
        );
        self.column_head(
            "PENGELUARAN",
            data.expense.len(),
            right_x,
            EXPENSE_INK,
            EXPENSE_BAND,
            y_top,
        );
        y_top - 17.0
    }

    fn column_head(&self, title: &str, count: usize, col_x: f32, ink: Ink, band: Ink, y_top: f32) {
        self.rect_fill(col_x, y_top - BAND_H, col_x + COL_W, y_top, band);
        self.text(
            &format!("{title}  ({count} transaksi)"),
            9.5,
            col_x + 2.5,
            y_top - 4.8,
            true,
            ink,
        );

        let sub_y = y_top - BAND_H - 4.0;
        self.text("TANGGAL", 7.0, col_x + 2.0, sub_y, true, INK_SOFT);
        self.text("DESKRIPSI", 7.0, col_x + 21.0, sub_y, true, INK_SOFT);
        self.text_right("JUMLAH (Rp)", 7.0, col_x + COL_W - 2.0, sub_y, true, INK_SOFT);
        self.hline(col_x, col_x + COL_W, sub_y - 2.0, 0.6, RULE);
    }

    /// Menggambar satu baris tabel (boleh kosong bila kolom terkait sudah habis).
    fn table_row(&self, y: f32, row: Option<&TransactionRow>, col_x: f32, ink: Ink) {
        if let Some(t) = row {
            self.text(
                &util::format_date_id(&t.date),
                8.5,
                col_x + 2.0,
                y,
                false,
                INK,
            );
            let deskripsi = t.description.as_deref().unwrap_or("-");
            self.text(&clip(deskripsi, DESC_MAX), 8.5, col_x + 21.0, y, false, INK_SOFT);
            self.text_right(
                &util::format_number(t.amount),
                8.5,
                col_x + COL_W - 2.0,
                y,
                false,
                ink,
            );
        }
        self.hline(col_x, col_x + COL_W, y - 1.8, 0.4, RULE);
    }

    /// Halaman lanjutan: kop ringkas + judul kolom.
    fn continuation_page(
        &mut self,
        periode: &str,
        data: &ReportData,
        left_x: f32,
        right_x: f32,
    ) -> f32 {
        self.add_page();
        self.text("Laporan Keuangan", 11.0, MARGIN_X, PAGE_H - 18.0, true, INK);
        self.text(
            &format!("Periode: {periode}"),
            8.5,
            MARGIN_X,
            PAGE_H - 24.0,
            false,
            INK_SOFT,
        );
        self.hline(MARGIN_X, PAGE_W - MARGIN_X, PAGE_H - 28.0, 0.7, RULE);
        self.column_heads(PAGE_H - 33.0, data, left_x, right_x)
    }

    /// Blok ringkasan di bawah tabel; mengembalikan `y` akhir.
    fn summary(&self, y: f32, data: &ReportData, left_x: f32, content_right: f32) -> f32 {
        let mut y = y;
        self.text("Ringkasan", 12.0, left_x, y, true, INK);
        y -= 7.5;

        let mut baris = |label: &str, nilai: f64, ink: Ink, bold: bool| {
            self.text(label, 9.5, left_x + 2.0, y, bold, INK_SOFT);
            self.text_right(
                &util::format_rupiah(nilai),
                9.5,
                content_right - 2.0,
                y,
                bold,
                ink,
            );
            y -= 6.5;
        };

        baris("Total Pemasukan", data.total_income, INCOME_INK, false);
        baris("Total Pengeluaran", data.total_expense, EXPENSE_INK, false);
        let net_ink = if data.net() >= 0.0 { INCOME_INK } else { EXPENSE_INK };
        baris("Selisih (Pemasukan \u{2212} Pengeluaran)", data.net(), net_ink, false);
        baris("Saldo Awal Periode", data.opening_balance, INK, false);

        // Baris "Sisa Saldo" diberi kotak agar menjadi penutup yang jelas.
        let saldo = data.closing_balance();
        let box_top = y + 4.5;
        let box_bottom = y - 3.5;
        self.rect_fill(left_x, box_bottom, content_right, box_top, SUMMARY_BG);
        self.rect_stroke(left_x, box_bottom, content_right, box_top, INK_SOFT, 0.7);
        let saldo_ink = if saldo >= 0.0 { INCOME_INK } else { EXPENSE_INK };
        self.text("Sisa Saldo", 10.5, left_x + 2.0, y, true, INK);
        self.text_right(
            &util::format_rupiah(saldo),
            10.5,
            content_right - 2.0,
            y,
            true,
            saldo_ink,
        );
        y - 3.5
    }

    /// Catatan kaki (identitas aplikasi + nomor halaman) di semua halaman.
    fn footers(&self, meta: &ReportMeta) {
        let total = self.pages.len();
        for (i, (page, layer)) in self.pages.iter().enumerate() {
            let layer = self.doc.get_page(*page).get_layer(*layer);
            let catatan = format!("Dibuat otomatis oleh {} v{}", meta.app_name, meta.version);
            layer.set_outline_color(Self::color(RULE));
            layer.set_outline_thickness(0.5);
            layer.add_line(Line {
                points: vec![
                    (Point::new(Mm(MARGIN_X), Mm(15.0)), false),
                    (Point::new(Mm(PAGE_W - MARGIN_X), Mm(15.0)), false),
                ],
                is_closed: false,
            });

            layer.set_fill_color(Self::color(INK_FAINT));
            layer.use_text(
                winansi(&catatan),
                8.0,
                Mm(MARGIN_X),
                Mm(10.5),
                &self.regular,
            );

            let nomor = format!("Halaman {} dari {}", i + 1, total);
            let bersih = winansi(&nomor);
            let x = PAGE_W - MARGIN_X - text_width_mm(&bersih, 8.0);
            layer.use_text(bersih, 8.0, Mm(x), Mm(10.5), &self.regular);
        }
    }
}

/// Menyesuaikan teks agar bisa dicetak font bawaan PDF (Windows-1252/Latin-1).
///
/// Karakter tipografi umum diterjemahkan ke padanan ASCII, sedangkan
/// karakter di luar jangkauan Latin-1 (mis. emoji) diganti `?` supaya
/// laporan tidak kehilangan baris.
fn winansi(teks: &str) -> String {
    let mut hasil = String::with_capacity(teks.len());
    for c in teks.chars() {
        let pengganti = match c {
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => '\'',
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{00AB}' | '\u{00BB}' => '"',
            '\u{2013}' | '\u{2014}' | '\u{2015}' => '-',
            '\u{2022}' | '\u{00B7}' => '-',
            '\u{2026}' => '.',
            '\u{2212}' => '-',
            '\u{00D7}' => 'x',
            '\n' | '\r' | '\t' => ' ',
            c if is_printable_latin1(c) => c,
            _ => '?',
        };
        hasil.push(pengganti);
    }
    hasil
}

/// Apakah karakter termasuk Latin-1 yang bisa dicetak (`0x20`–`0x7E`, `0xA0`–`0xFF`).
fn is_printable_latin1(c: char) -> bool {
    let kode = c as u32;
    (0x20..=0x7E).contains(&kode) || (0xA0..=0xFF).contains(&kode)
}

/// Memotong teks agar muat di kolom deskripsi (elipsis dibuat ASCII).
fn clip(teks: &str, max: usize) -> String {
    let bersih = winansi(teks);
    let jumlah = bersih.chars().count();
    if jumlah <= max {
        return bersih;
    }
    let ambil: String = bersih.chars().take(max.saturating_sub(2)).collect();
    format!("{ambil}..")
}

/// Lebar teks (mm) untuk font Helvetica pada ukuran `size_pt`.
///
/// Tabel lebar hanya mencakup karakter yang dipakai pada teks rata kanan
/// (angka, titik, minus, dan huruf pada judul "JUMLAH (Rp)"). Karakter lain
/// memakai lebar rata-rata, cukup untuk keperluan perataan ini.
fn text_width_mm(teks: &str, size_pt: f32) -> f32 {
    let em: f32 = teks
        .chars()
        .map(|c| match c {
            '0'..='9' => 556.0,
            '.' | ',' | '/' | ':' | ' ' => 278.0,
            '-' => 333.0,
            '(' | ')' => 333.0,
            'R' | 'r' => 667.0,
            'p' => 556.0,
            'J' | 'j' => 500.0,
            'U' | 'u' => 722.0,
            'A' | 'a' => 667.0,
            'H' | 'h' => 722.0,
            'L' | 'l' => 222.0,
            _ => 556.0,
        })
        .sum();
    em / 1000.0 * size_pt * 0.352_777_8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controllers::ReportController;
    use crate::db::schema;
    use crate::models::{ReportPeriod, Transaction};
    use rusqlite::Connection;

    fn db_dengan_transaksi() -> Connection {
        let conn = Connection::open_in_memory().expect("buka database");
        schema::init(&conn).expect("siapkan skema");
        Transaction::create(
            &conn,
            1,
            1,
            5_000_000.0,
            "income",
            Some("Gaji bulanan"),
            "2026-01-05",
        )
        .expect("pemasukan");
        Transaction::create(
            &conn,
            4,
            1,
            125_000.0,
            "expense",
            Some("Makan siang bersama tim"),
            "2026-01-06",
        )
        .expect("pengeluaran");
        conn
    }

    fn meta() -> ReportMeta {
        ReportMeta {
            app_name: "Lacakuang".to_string(),
            version: "1.0.0".to_string(),
            currency_code: "IDR".to_string(),
            author: "Penguji".to_string(),
        }
    }

    #[test]
    fn default_report_path_berakhir_pdf() {
        let dir = std::path::Path::new("/tmp/opencode/laporan");
        let path = default_report_path(dir);
        assert_eq!(path.parent(), Some(dir));
        let nama = path.file_name().unwrap().to_string_lossy();
        assert!(nama.starts_with("laporan-"), "nama: {nama}");
        assert!(nama.ends_with(".pdf"), "nama: {nama}");
    }

    #[test]
    fn menulis_laporan_dua_kolom() {
        let conn = db_dengan_transaksi();
        let data =
            ReportController::build(&conn, ReportPeriod::All).expect("susun data laporan");
        assert_eq!(data.income.len(), 1);
        assert_eq!(data.expense.len(), 1);
        assert_eq!(data.total_income, 5_000_000.0);
        assert_eq!(data.total_expense, 125_000.0);

        let target = std::path::Path::new("/tmp/opencode/laporan-uji.pdf");
        let _ = std::fs::remove_file(target);
        write_laporan(target, &data, &meta()).expect("tulis laporan");

        let ukuran = std::fs::metadata(target).expect("berkas ada").len();
        assert!(ukuran > 1_000, "berkas PDF terlalu kecil: {ukuran} byte");

        // PDF harus diawali penanda %PDF.
        let isi = std::fs::read(target).expect("baca berkas");
        assert!(isi.starts_with(b"%PDF"), "berkas bukan PDF");
    }

    #[test]
    fn laporan_kosong_ditolak() {
        let conn = Connection::open_in_memory().expect("buka database");
        schema::init(&conn).expect("siapkan skema");
        let data =
            ReportController::build(&conn, ReportPeriod::All).expect("susun data laporan");
        assert!(data.income.is_empty() && data.expense.is_empty());

        let target = std::path::Path::new("/tmp/opencode/laporan-kosong.pdf");
        let _ = std::fs::remove_file(target);
        let hasil = write_laporan(target, &data, &meta());
        assert!(hasil.is_err(), "laporan kosong seharusnya ditolak");
        assert!(!target.exists(), "berkas tidak boleh dibuat");
    }
}
