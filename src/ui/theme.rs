//! Tema visual: palet warna TailwindCSS dan konfigurasi gaya egui.
//!
//! Warna-warna di bawah ini sengaja memakai nilai heksadesimal yang sama
//! dengan palet Tailwind yang dipakai versi PHP, supaya tampilan identik.

use eframe::egui::{
    self, Color32, FontId, Shadow, Stroke, TextStyle,
};

// ---- Gray ----
pub const GRAY_50: Color32 = Color32::from_rgb(0xF9, 0xFA, 0xFB);
pub const GRAY_100: Color32 = Color32::from_rgb(0xF3, 0xF4, 0xF6);
pub const GRAY_200: Color32 = Color32::from_rgb(0xE5, 0xE7, 0xEB);
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

/// Beberapa pilihan warna untuk form kategori (sama persis dengan versi PHP).
pub const CATEGORY_PALETTE: [&str; 8] = [
    "#EF4444", "#F59E0B", "#EC4899", "#8B5CF6", "#3B82F6", "#10B981", "#6366F1", "#6B7280",
];

/// Menerapkan palet & gaya dasar ke konteks egui.
pub fn configure(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();

    visuals.panel_fill = GRAY_50;
    visuals.window_fill = Color32::WHITE;
    visuals.extreme_bg_color = Color32::WHITE;
    visuals.faint_bg_color = GRAY_50;
    visuals.selection.bg_fill = BLUE_100;
    visuals.selection.stroke = Stroke::new(1.0, PRIMARY);
    // Warna tautan disamakan dengan warna utama aplikasi (bukan biru bawaan egui).
    visuals.hyperlink_color = BLUE_600;
    visuals.window_corner_radius = egui::CornerRadius::same(12);
    visuals.window_stroke = Stroke::new(1.0, GRAY_200);
    visuals.window_shadow = Shadow {
        color: Color32::from_black_alpha(40),
        ..Default::default()
    };
    visuals.popup_shadow = Shadow {
        color: Color32::from_black_alpha(30),
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

    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, GRAY_200);
    visuals.widgets.noninteractive.corner_radius = radius;

    visuals.widgets.inactive.bg_fill = Color32::WHITE;
    visuals.widgets.inactive.weak_bg_fill = Color32::WHITE;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, GRAY_300);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, GRAY_700);

    visuals.widgets.hovered.bg_fill = GRAY_100;
    visuals.widgets.hovered.weak_bg_fill = GRAY_100;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, GRAY_400);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, GRAY_900);

    visuals.widgets.active.bg_fill = BLUE_50;
    visuals.widgets.active.weak_bg_fill = BLUE_50;
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, PRIMARY);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, GRAY_900);

    ctx.set_theme(egui::ThemePreference::Light);
    ctx.set_visuals(visuals);

    ctx.style_mut_of(egui::Theme::Light, |style| {
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.interact_size = egui::vec2(0.0, 32.0);
        style.spacing.window_margin = egui::Margin::same(16);

        style.text_styles.insert(TextStyle::Body, FontId::proportional(15.0));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(14.0));
        style.text_styles.insert(TextStyle::Small, FontId::proportional(12.0));
        style.text_styles.insert(TextStyle::Heading, FontId::proportional(20.0));
        style.text_styles.insert(TextStyle::Monospace, FontId::monospace(14.0));
    });
}
