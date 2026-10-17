//! Tema visual: palet warna TailwindCSS dan konfigurasi gaya egui.
//!
//! Warna-warna di bawah ini sengaja memakai nilai heksadesimal yang sama
//! dengan palet Tailwind yang dipakai versi PHP, supaya tampilan identik.
//!
//! Sejak dukungan mode gelap ditambahkan, warna **tidak lagi dipakai langsung**
//! dari konstanta di bawah. Kode UI memakai fungsi semantik seperti
//! [`text`], [`surface`], atau [`border`] yang otomatis menyesuaikan diri
//! dengan tema aktif ([`ThemeKind`]). Konstanta seperti [`GRAY_900`] tetap
//! dipertahankan karena masih dipakai sebagai bahan penyusun palet.

use eframe::egui::{self, Color32, FontId, Shadow, Stroke, TextStyle};

use std::sync::atomic::{AtomicU8, Ordering};

// ---- Gray ----
pub const GRAY_50: Color32 = Color32::from_rgb(0xF9, 0xFA, 0xFB);
pub const GRAY_100: Color32 = Color32::from_rgb(0xF3, 0xF4, 0xF6);
pub const GRAY_200: Color32 = Color32::from_rgb(0xE5, 0xE7, 0xEB);
#[allow(dead_code)]
pub const GRAY_300: Color32 = Color32::from_rgb(0xD1, 0xD5, 0xDB);
pub const GRAY_400: Color32 = Color32::from_rgb(0x9C, 0xA3, 0xAF);
pub const GRAY_500: Color32 = Color32::from_rgb(0x6B, 0x72, 0x80);
pub const GRAY_600: Color32 = Color32::from_rgb(0x4B, 0x55, 0x63);
pub const GRAY_700: Color32 = Color32::from_rgb(0x37, 0x41, 0x51);
pub const GRAY_800: Color32 = Color32::from_rgb(0x1F, 0x29, 0x37);
pub const GRAY_900: Color32 = Color32::from_rgb(0x11, 0x18, 0x27);

// ---- Primary (blue) ----
pub const PRIMARY: Color32 = Color32::from_rgb(0x3B, 0x82, 0xF6);
pub const BLUE_50: Color32 = Color32::from_rgb(0xEF, 0xF6, 0xFF);
pub const BLUE_100: Color32 = Color32::from_rgb(0xDB, 0xEA, 0xFE);
pub const BLUE_500: Color32 = Color32::from_rgb(0x3B, 0x82, 0xF6);
pub const BLUE_600: Color32 = Color32::from_rgb(0x25, 0x63, 0xEB);
pub const BLUE_700: Color32 = Color32::from_rgb(0x1D, 0x4E, 0xD8);
pub const BLUE_800: Color32 = Color32::from_rgb(0x1E, 0x40, 0xAF);
pub const BLUE_900: Color32 = Color32::from_rgb(0x1E, 0x3A, 0x8A);

// ---- Success (green) ----
pub const GREEN_50: Color32 = Color32::from_rgb(0xEC, 0xFD, 0xF5);
pub const GREEN_100: Color32 = Color32::from_rgb(0xD1, 0xFA, 0xE5);
pub const GREEN_500: Color32 = Color32::from_rgb(0x10, 0xB9, 0x81);
pub const GREEN_600: Color32 = Color32::from_rgb(0x05, 0x96, 0x69);
pub const GREEN_800: Color32 = Color32::from_rgb(0x06, 0x5F, 0x46);

// ---- Danger (red) ----
pub const RED_50: Color32 = Color32::from_rgb(0xFE, 0xF2, 0xF2);
pub const RED_100: Color32 = Color32::from_rgb(0xFE, 0xE2, 0xE2);
pub const RED_500: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44);
pub const RED_600: Color32 = Color32::from_rgb(0xDC, 0x26, 0x26);
pub const RED_700: Color32 = Color32::from_rgb(0xB9, 0x1C, 0x1C);
pub const RED_800: Color32 = Color32::from_rgb(0x99, 0x1B, 0x1B);

// ---- Purple ----
pub const PURPLE_100: Color32 = Color32::from_rgb(0xED, 0xE9, 0xFE);
pub const PURPLE_500: Color32 = Color32::from_rgb(0x8B, 0x5C, 0xF6);
pub const PURPLE_800: Color32 = Color32::from_rgb(0x5B, 0x21, 0xB6);

// ---- Varian khusus mode gelap ----
//
// Warna di atas dirancang untuk latar terang. Saat latar berubah gelap,
// teks yang tadinya gelap harus menjadi terang, dan latar "soft" harus
// menjadi versi gelap agar tetap kontras. Nilai di bawah adalah padanan
// Tailwind untuk keperluan tersebut (blue-400, emerald-300, red-300, dst.).
const DARK_SURFACE_ALT: Color32 = Color32::from_rgb(0x2A, 0x35, 0x44);
const DARK_INPUT_BG: Color32 = Color32::from_rgb(0x0F, 0x17, 0x24);
const PRIMARY_TEXT_DARK: Color32 = Color32::from_rgb(0x60, 0xA5, 0xFA);
const PRIMARY_TEXT_HOVER_DARK: Color32 = Color32::from_rgb(0x93, 0xC5, 0xFD);
const INCOME_TEXT_DARK: Color32 = Color32::from_rgb(0x34, 0xD3, 0x99);
const EXPENSE_TEXT_DARK: Color32 = Color32::from_rgb(0xF8, 0x71, 0x71);
const INCOME_BADGE_BG_DARK: Color32 = Color32::from_rgb(0x06, 0x4E, 0x3B);
const INCOME_BADGE_FG_DARK: Color32 = Color32::from_rgb(0x6E, 0xE7, 0xB7);
const EXPENSE_BADGE_BG_DARK: Color32 = Color32::from_rgb(0x7F, 0x1D, 0x1D);
const EXPENSE_BADGE_FG_DARK: Color32 = Color32::from_rgb(0xFC, 0xA5, 0xA5);
const INFO_BADGE_BG_DARK: Color32 = Color32::from_rgb(0x1E, 0x3A, 0x8A);
const INFO_BADGE_FG_DARK: Color32 = Color32::from_rgb(0x93, 0xC5, 0xFD);
const PURPLE_BADGE_BG_DARK: Color32 = Color32::from_rgb(0x4C, 0x1D, 0x95);
const PURPLE_BADGE_FG_DARK: Color32 = Color32::from_rgb(0xC4, 0xB5, 0xFD);
const PRIMARY_SOFT_DARK: Color32 = Color32::from_rgb(0x1E, 0x3A, 0x8A);
const INCOME_SOFT_DARK: Color32 = Color32::from_rgb(0x06, 0x4E, 0x3B);
const EXPENSE_SOFT_DARK: Color32 = Color32::from_rgb(0x7F, 0x1D, 0x1D);

/// Beberapa pilihan warna untuk form kategori (sama persis dengan versi PHP).
pub const CATEGORY_PALETTE: [&str; 8] = [
    "#EF4444", "#F59E0B", "#EC4899", "#8B5CF6", "#3B82F6", "#10B981", "#6366F1", "#6B7280",
];

// ---------------------------------------------------------------
// Tema aktif
// ---------------------------------------------------------------

/// Pilihan tema tampilan aplikasi.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeKind {
    Light,
    Dark,
}

impl ThemeKind {
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Tema hasil tombol "ganti tema" ditekan.
    pub fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    /// Ikon yang ditampilkan pada tombol ganti tema (matahari/bulan).
    pub fn icon(self) -> &'static str {
        match self {
            // Tema terang aktif -> tawarkan mode gelap (ikon bulan).
            Self::Light => "\u{1F319}",
            Self::Dark => "\u{2600}\u{FE0F}",
        }
    }

    /// Keterangan pada tooltip tombol ganti tema.
    pub fn switch_label(self) -> &'static str {
        match self {
            Self::Light => "Ganti ke mode gelap",
            Self::Dark => "Ganti ke mode terang",
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            Self::Light => 0,
            Self::Dark => 1,
        }
    }

    fn from_u8(value: u8) -> Self {
        if value == 1 {
            Self::Dark
        } else {
            Self::Light
        }
    }
}

/// Tema yang sedang aktif. Dipakai oleh seluruh fungsi palet di bawah.
static CURRENT_THEME: AtomicU8 = AtomicU8::new(0);

/// Tema yang sedang aktif.
pub fn current() -> ThemeKind {
    ThemeKind::from_u8(CURRENT_THEME.load(Ordering::Relaxed))
}

/// Menetapkan tema aktif (tanpa mengubah gaya egui; lihat [`apply`]).
pub fn set_current(kind: ThemeKind) {
    CURRENT_THEME.store(kind.as_u8(), Ordering::Relaxed);
}

/// Memilih warna sesuai tema aktif.
fn pick(light: Color32, dark: Color32) -> Color32 {
    if current().is_dark() {
        dark
    } else {
        light
    }
}

// ---------------------------------------------------------------
// Palet semantik (menyesuaikan tema aktif)
// ---------------------------------------------------------------

/// Latar halaman utama.
pub fn page_bg() -> Color32 {
    pick(GRAY_50, GRAY_900)
}

/// Latar kartu, navbar, footer, modal, dan kolom input.
pub fn surface() -> Color32 {
    pick(Color32::WHITE, GRAY_800)
}

/// Permukaan sekunder: header tabel, chip, dan sorotan baris.
pub fn surface_alt() -> Color32 {
    pick(GRAY_50, DARK_SURFACE_ALT)
}

/// Latar kolom input teks.
pub fn input_bg() -> Color32 {
    pick(Color32::WHITE, DARK_INPUT_BG)
}

/// Garis pemisah (border) yang jelas terlihat.
pub fn border() -> Color32 {
    pick(GRAY_200, GRAY_700)
}

/// Garis tepi halus (tepi kartu/modal).
pub fn border_soft() -> Color32 {
    pick(GRAY_100, DARK_SURFACE_ALT)
}

/// Jalur (track) progress bar.
pub fn track() -> Color32 {
    pick(GRAY_200, GRAY_700)
}

/// Warna teks utama.
pub fn text() -> Color32 {
    pick(GRAY_900, GRAY_50)
}

/// Warna teks sekunder (label, menu non-aktif).
pub fn text_soft() -> Color32 {
    pick(GRAY_700, GRAY_200)
}

/// Warna teks keterangan kecil.
pub fn text_muted() -> Color32 {
    pick(GRAY_500, GRAY_400)
}

/// Warna teks paling redup (footer, ikon keadaan kosong).
pub fn text_faint() -> Color32 {
    pick(GRAY_400, GRAY_500)
}

// ---- Tombol utama (biru) ----

/// Latar tombol utama.
pub fn primary_fill() -> Color32 {
    PRIMARY
}

/// Latar tombol utama saat disorot.
pub fn primary_fill_hover() -> Color32 {
    pick(BLUE_700, BLUE_600)
}

/// Warna teks/tautan biru.
pub fn primary_text() -> Color32 {
    pick(PRIMARY, PRIMARY_TEXT_DARK)
}

/// Warna teks/tautan biru saat disorot.
pub fn primary_text_hover() -> Color32 {
    pick(BLUE_900, PRIMARY_TEXT_HOVER_DARK)
}

/// Latar biru muda (menu aktif, tombol halaman aktif).
pub fn primary_soft() -> Color32 {
    pick(BLUE_50, PRIMARY_SOFT_DARK)
}

/// Latar lencana (badge) biru.
pub fn primary_badge_bg() -> Color32 {
    pick(BLUE_100, INFO_BADGE_BG_DARK)
}

/// Warna teks lencana biru.
pub fn primary_badge_fg() -> Color32 {
    pick(BLUE_800, INFO_BADGE_FG_DARK)
}

// ---- Tombol sekunder (abu-abu) ----

/// Latar tombol sekunder.
pub fn secondary_fill() -> Color32 {
    pick(GRAY_100, GRAY_700)
}

/// Latar tombol sekunder saat disorot.
pub fn secondary_fill_hover() -> Color32 {
    pick(GRAY_200, GRAY_600)
}

/// Warna teks tombol sekunder.
pub fn secondary_text() -> Color32 {
    pick(GRAY_700, GRAY_100)
}

// ---- Pemasukan (hijau) ----

/// Warna teks jumlah pemasukan.
pub fn income_text() -> Color32 {
    pick(GREEN_600, INCOME_TEXT_DARK)
}

/// Latar tombol/aksen pemasukan berwarna penuh.
pub fn income_fill() -> Color32 {
    GREEN_600
}

/// Latar lencana pemasukan.
pub fn income_badge_bg() -> Color32 {
    pick(GREEN_100, INCOME_BADGE_BG_DARK)
}

/// Warna teks lencana pemasukan.
pub fn income_badge_fg() -> Color32 {
    pick(GREEN_800, INCOME_BADGE_FG_DARK)
}

/// Latar hijau sangat muda (kartu pilihan tipe).
pub fn income_soft() -> Color32 {
    pick(GREEN_50, INCOME_SOFT_DARK)
}

/// Warna aksen hijau (garis kartu statistik, progress bar).
pub fn income_accent() -> Color32 {
    GREEN_500
}

// ---- Pengeluaran (merah) ----

/// Warna teks jumlah pengeluaran.
pub fn expense_text() -> Color32 {
    pick(RED_600, EXPENSE_TEXT_DARK)
}

/// Latar tombol/aksen pengeluaran berwarna penuh.
pub fn expense_fill() -> Color32 {
    RED_600
}

/// Latar lencana pengeluaran.
pub fn expense_badge_bg() -> Color32 {
    pick(RED_100, EXPENSE_BADGE_BG_DARK)
}

/// Warna teks lencana pengeluaran.
pub fn expense_badge_fg() -> Color32 {
    pick(RED_800, EXPENSE_BADGE_FG_DARK)
}

/// Latar merah sangat muda (kartu pilihan tipe, kotak galat).
pub fn expense_soft() -> Color32 {
    pick(RED_50, EXPENSE_SOFT_DARK)
}

/// Warna aksen merah (garis kartu statistik, progress bar).
pub fn expense_accent() -> Color32 {
    RED_500
}

// ---- Bahaya/hapus (merah) ----

/// Latar tombol hapus.
pub fn danger_fill() -> Color32 {
    RED_600
}

/// Latar tombol hapus saat disorot.
pub fn danger_fill_hover() -> Color32 {
    RED_700
}

/// Warna teks tautan hapus.
pub fn danger_text() -> Color32 {
    pick(RED_600, EXPENSE_TEXT_DARK)
}

/// Warna teks tautan hapus saat disorot.
pub fn danger_text_hover() -> Color32 {
    pick(RED_800, EXPENSE_BADGE_FG_DARK)
}

// ---- Lencana netral & ungu ----

/// Latar lencana netral (tipe akun "lainnya").
pub fn neutral_badge_bg() -> Color32 {
    pick(GRAY_100, GRAY_700)
}

/// Warna teks lencana netral.
pub fn neutral_badge_fg() -> Color32 {
    pick(GRAY_800, GRAY_200)
}

/// Latar lencana ungu (e-wallet).
pub fn purple_badge_bg() -> Color32 {
    pick(PURPLE_100, PURPLE_BADGE_BG_DARK)
}

/// Warna teks lencana ungu.
pub fn purple_badge_fg() -> Color32 {
    pick(PURPLE_800, PURPLE_BADGE_FG_DARK)
}

// ---------------------------------------------------------------
// Penerapan tema ke egui
// ---------------------------------------------------------------

/// Menyusun [`egui::Visuals`] untuk tema aktif.
fn build_visuals() -> egui::Visuals {
    let dark = current().is_dark();
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.panel_fill = page_bg();
    visuals.window_fill = surface();
    visuals.extreme_bg_color = input_bg();
    visuals.faint_bg_color = surface_alt();
    visuals.code_bg_color = surface_alt();
    visuals.selection.bg_fill = primary_soft();
    visuals.selection.stroke = Stroke::new(1.0, primary_fill());
    // Warna tautan disamakan dengan warna utama aplikasi (bukan biru bawaan egui).
    visuals.hyperlink_color = primary_text();
    visuals.window_corner_radius = egui::CornerRadius::same(12);
    visuals.window_stroke = Stroke::new(1.0, border());
    visuals.window_shadow = Shadow {
        color: Color32::from_black_alpha(if dark { 120 } else { 40 }),
        ..Default::default()
    };
    visuals.popup_shadow = Shadow {
        color: Color32::from_black_alpha(if dark { 100 } else { 30 }),
        ..Default::default()
    };

    let radius = egui::CornerRadius::same(8);
    for w in [
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        w.corner_radius = radius;
    }

    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border());
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text());
    visuals.widgets.noninteractive.corner_radius = radius;

    visuals.widgets.inactive.bg_fill = surface();
    visuals.widgets.inactive.weak_bg_fill = surface();
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border());
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_soft());

    visuals.widgets.hovered.bg_fill = secondary_fill_hover();
    visuals.widgets.hovered.weak_bg_fill = secondary_fill_hover();
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, border());
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, text());

    visuals.widgets.active.bg_fill = primary_soft();
    visuals.widgets.active.weak_bg_fill = primary_soft();
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, primary_fill());
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, text());

    visuals
}

/// Menerapkan tema (palet + gaya dasar) ke konteks egui.
///
/// Aman dipanggil berulang kali, misalnya saat pengguna menekan tombol
/// ganti tema.
pub fn apply(ctx: &egui::Context, kind: ThemeKind) {
    // Palet harus sudah menunjuk tema yang benar sebelum `visuals` disusun.
    set_current(kind);

    ctx.set_theme(match kind {
        ThemeKind::Light => egui::ThemePreference::Light,
        ThemeKind::Dark => egui::ThemePreference::Dark,
    });
    ctx.set_visuals_of(
        match kind {
            ThemeKind::Light => egui::Theme::Light,
            ThemeKind::Dark => egui::Theme::Dark,
        },
        build_visuals(),
    );

    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        ctx.style_mut_of(theme, |style| {
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
            style.spacing.interact_size = egui::vec2(0.0, 32.0);
            style.spacing.window_margin = egui::Margin::same(16);

            style
                .text_styles
                .insert(TextStyle::Body, FontId::proportional(15.0));
            style
                .text_styles
                .insert(TextStyle::Button, FontId::proportional(14.0));
            style
                .text_styles
                .insert(TextStyle::Small, FontId::proportional(12.0));
            style
                .text_styles
                .insert(TextStyle::Heading, FontId::proportional(20.0));
            style
                .text_styles
                .insert(TextStyle::Monospace, FontId::monospace(14.0));
        });
    }
}

// ---------------------------------------------------------------
// Preferensi tema tersimpan
// ---------------------------------------------------------------

/// Membaca tema yang terakhir dipakai dari berkas preferensi.
///
/// Bila berkas belum ada atau isinya tidak dikenal, tema terang dipakai.
pub fn load_saved() -> ThemeKind {
    match std::fs::read_to_string(crate::config::resolve_theme_path()) {
        Ok(isi) => {
            if isi.trim().eq_ignore_ascii_case("dark") {
                ThemeKind::Dark
            } else {
                ThemeKind::Light
            }
        }
        Err(_) => ThemeKind::Light,
    }
}

/// Menyimpan pilihan tema ke berkas preferensi (kegagalan diabaikan, karena
/// ini hanya kenyamanan dan tidak boleh mengganggu jalannya aplikasi).
pub fn save_preference(kind: ThemeKind) {
    let path = crate::config::resolve_theme_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let isi = match kind {
        ThemeKind::Dark => "dark",
        ThemeKind::Light => "light",
    };
    let _ = std::fs::write(path, isi);
}
