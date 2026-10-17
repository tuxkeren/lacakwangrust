//! Halaman Manajemen Kategori — padanan dari `views/categories/index.php`.

use eframe::egui::{self, Align, Color32, Layout, RichText, Sense, Stroke};

use crate::controllers::CategoryController;
use crate::models::Category;
use crate::util;

use super::app::{App, CatForm, Confirm, ConfirmAction};
use super::widgets::{self, ButtonStyle};
use super::theme;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let data = match CategoryController::index(app.db.conn()) {
        Ok(d) => d,
        Err(e) => {
            widgets::error_box(ui, &format!("Gagal memuat kategori: {e}"));
            return;
        }
    };

    ui.horizontal(|ui| {
        widgets::page_title(ui, "Manajemen Kategori");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if widgets::styled_button(ui, "+ Tambah Kategori", ButtonStyle::primary(), None).clicked()
            {
                app.cat_form = Some(new_form());
            }
            ui.add_space(12.0);
            widgets::muted(ui, &format!("{} kategori terdaftar", data.categories.len()));
        });
    });
    ui.add_space(18.0);

    ui.columns(2, |cols| {
        category_group(
            &mut cols[0],
            "Kategori Pemasukan",
            theme::income_accent(),
            &data.income_categories,
            "Belum ada kategori pemasukan",
            app,
        );
        category_group(
            &mut cols[1],
            "Kategori Pengeluaran",
            theme::expense_accent(),
            &data.expense_categories,
            "Belum ada kategori pengeluaran",
            app,
        );
    });
}

/// Form kosong untuk kategori baru.
fn new_form() -> CatForm {
    CatForm {
        id: None,
        name: String::new(),
        color: "#3B82F6".to_string(),
        kind: "expense".to_string(),
        error: None,
    }
}

fn category_group(
    ui: &mut egui::Ui,
    title: &str,
    dot_color: Color32,
    categories: &[Category],
    empty_text: &str,
    app: &mut App,
) {
    widgets::card(ui, |ui| {
        ui.horizontal(|ui| {
            widgets::dot(ui, dot_color);
            widgets::section_title(ui, title);
        });
        ui.add_space(12.0);

        if categories.is_empty() {
            widgets::muted(ui, empty_text);
            return;
        }

        for chunk in categories.chunks(3) {
            ui.columns(3, |cols| {
                for (i, cat) in chunk.iter().enumerate() {
                    category_chip(&mut cols[i], cat, app);
                }
            });
            ui.add_space(8.0);
        }
    });
}

fn category_chip(ui: &mut egui::Ui, cat: &Category, app: &mut App) {
    egui::Frame::new()
        .fill(theme::surface_alt())
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                widgets::dot(ui, util::hex_to_color(&cat.color));
                ui.label(
                    RichText::new(util::truncate(&cat.name, 14))
                        .size(13.0)
                        .strong()
                        .color(theme::text()),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if widgets::link(ui, "\u{1F5D1}", theme::danger_text(), theme::danger_text_hover()).clicked() {
                        app.confirm = Some(Confirm {
                            title: "Hapus Kategori".into(),
                            message: format!(
                                "Yakin ingin menghapus kategori \"{}\"?",
                                cat.name
                            ),
                            action: ConfirmAction::DeleteCategory(cat.id),
                        });
                    }
                    if widgets::link(ui, "\u{270F}", theme::primary_text(), theme::primary_text_hover()).clicked() {
                        app.open_cat_edit(cat.id);
                    }
                });
            });
        });
}

/// Membuka modal form kategori.
pub fn show_form(app: &mut App, ctx: &egui::Context) {
    let Some(mut form) = app.cat_form.take() else {
        return;
    };

    let mut close = false;
    let mut save = false;

    widgets::modal(ctx, "cat_form_modal", egui::vec2(470.0, 500.0), |ui| {
        let is_edit = form.id.is_some();
        ui.label(
            RichText::new(if is_edit { "Edit Kategori" } else { "Tambah Kategori" })
                .size(20.0)
                .strong()
                .color(theme::text()),
        );
        ui.add_space(16.0);

        // Tipe kategori
        widgets::field_label(ui, "Tipe Kategori");
        ui.add_space(6.0);
        let gap = 12.0;
        let w = (ui.available_width() - gap) / 2.0;
        ui.horizontal(|ui| {
            if widgets::choice_card(
                ui,
                "Pemasukan",
                None,
                form.kind == "income",
                w,
                theme::income_accent(),
                theme::income_soft(),
            )
            .clicked()
            {
                form.kind = "income".into();
            }
            ui.add_space(gap);
            if widgets::choice_card(
                ui,
                "Pengeluaran",
                None,
                form.kind == "expense",
                w,
                theme::expense_accent(),
                theme::expense_soft(),
            )
            .clicked()
            {
                form.kind = "expense".into();
            }
        });
        ui.add_space(14.0);

        // Nama kategori
        widgets::field_label(ui, "Nama Kategori");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.name, "Contoh: Makanan", field_width);
        ui.add_space(14.0);

        // Warna
        widgets::field_label(ui, "Warna");
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            for hex in theme::CATEGORY_PALETTE {
                if swatch(ui, hex, form.color.eq_ignore_ascii_case(hex)).clicked() {
                    form.color = hex.to_string();
                }
                ui.add_space(6.0);
            }
        });
        ui.add_space(8.0);
        widgets::muted(ui, "Atau masukkan kode warna lain (contoh: #3B82F6):");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.color, "#3B82F6", field_width);
        ui.add_space(16.0);

        if let Some(err) = &form.error {
            widgets::error_box(ui, err);
            ui.add_space(12.0);
        }

        let gap = 12.0;
        let total = ui.available_width();
        let primary_w = total * 0.55;
        let secondary_w = total - primary_w - gap;
        ui.horizontal(|ui| {
            if widgets::styled_button(ui, "Simpan", ButtonStyle::primary(), Some(primary_w)).clicked() {
                save = true;
            }
            ui.add_space(gap);
            if widgets::styled_button(ui, "Batal", ButtonStyle::secondary(), Some(secondary_w)).clicked()
            {
                close = true;
            }
        });
    });

    if save {
        match CategoryController::save(app.db.conn(), form.id, &form.name, &form.kind, &form.color) {
            Ok(()) => {
                app.notify_success(if form.id.is_some() {
                    "Kategori berhasil diperbarui"
                } else {
                    "Kategori berhasil ditambahkan"
                });
            }
            Err(message) => {
                form.error = Some(message);
                app.cat_form = Some(form);
            }
        }
        return;
    }

    if !close {
        app.cat_form = Some(form);
    }
}

/// Tombol pilihan warna bulat.
fn swatch(ui: &mut egui::Ui, hex: &str, selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(30.0, 30.0), Sense::click());
    let color = util::hex_to_color(hex);
    ui.painter().circle_filled(rect.center(), 12.0, color);
    if selected {
        ui.painter()
            .circle_stroke(rect.center(), 14.0, Stroke::new(2.0, theme::text_soft()));
    }
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}
