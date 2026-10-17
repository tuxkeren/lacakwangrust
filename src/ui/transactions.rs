//! Halaman Transaksi (daftar + form tambah/edit).
//!
//! Padanan dari `views/transactions/index.php` dan `views/transactions/form.php`.

use eframe::egui::{self, Align, Color32, Layout, RichText};

use crate::controllers::{TransactionController, TransactionFilter};
use crate::util;

use super::app::{App, Confirm, ConfirmAction, TxForm};
use super::widgets::{self, ButtonStyle, CellAlign};
use super::theme;

/// Proporsi lebar setiap kolom tabel (total = 1.0).
const COLUMNS: [(&str, f32, CellAlign); 7] = [
    ("TANGGAL", 0.12, CellAlign::Left),
    ("KATEGORI", 0.17, CellAlign::Left),
    ("AKUN", 0.14, CellAlign::Left),
    ("DESKRIPSI", 0.22, CellAlign::Left),
    ("TIPE", 0.10, CellAlign::Left),
    ("JUMLAH", 0.13, CellAlign::Right),
    ("AKSI", 0.12, CellAlign::Right),
];

const ROW_HEIGHT: f32 = 52.0;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let data = match TransactionController::index(
        app.db.conn(),
        app.tx_filter,
        app.tx_page,
        app.config.per_page,
    ) {
        Ok(d) => d,
        Err(e) => {
            widgets::error_box(ui, &format!("Gagal memuat transaksi: {e}"));
            return;
        }
    };
    app.tx_page = data.current_page;

    // Rentang baris yang tampil, dipakai pada keterangan paginasi.
    let row_start = data.current_page.saturating_sub(1) * data.per_page + 1;
    let row_end = (row_start + data.per_page - 1).min(data.total_count.max(0) as usize);

    // ---- Header ----
    ui.horizontal(|ui| {
        widgets::page_title(ui, "Daftar Transaksi");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if widgets::styled_button(ui, "+ Tambah Transaksi", ButtonStyle::primary(), None).clicked()
            {
                app.open_tx_form();
            }
        });
    });
    ui.add_space(18.0);

    // ---- Filter ----
    widgets::card(ui, |ui| {
        ui.horizontal(|ui| {
            for filter in [
                TransactionFilter::All,
                TransactionFilter::Income,
                TransactionFilter::Expense,
            ] {
                let active = app.tx_filter == filter;
                let accent = match filter {
                    TransactionFilter::All => theme::primary_fill(),
                    TransactionFilter::Income => theme::income_fill(),
                    TransactionFilter::Expense => theme::expense_fill(),
                };
                let style = if active {
                    ButtonStyle {
                        fill: accent,
                        hover: accent,
                        text: Color32::WHITE,
                    }
                } else {
                    ButtonStyle {
                        fill: theme::secondary_fill(),
                        hover: theme::secondary_fill_hover(),
                        text: theme::secondary_text(),
                    }
                };
                if widgets::styled_button(ui, filter.label(), style, None).clicked() {
                    app.tx_filter = filter;
                    app.tx_page = 1;
                }
                ui.add_space(6.0);
            }
        });
    });

    ui.add_space(20.0);

    // ---- Tabel ----
    widgets::card_flush(ui, |ui| {
        if data.transactions.is_empty() {
            ui.add_space(8.0);
            if widgets::empty_state(
                ui,
                "\u{1F4C4}",
                "Belum ada transaksi",
                "Mulai dengan menambahkan transaksi pertama Anda.",
                Some("+ Tambah Transaksi"),
            ) {
                app.open_tx_form();
            }
            ui.add_space(8.0);
            return;
        }

        // Baris tabel dibuat rapat (tanpa jarak antar-item bawaan egui),
        // supaya garis pemisah antar baris bersambung seperti versi PHP.
        ui.spacing_mut().item_spacing.y = 0.0;

        widgets::table_header(ui, &COLUMNS);

        for t in &data.transactions {
            let row_id = t.id;
            widgets::table_row(ui, ROW_HEIGHT, &COLUMNS, |ui, cols| {
                // Tanggal
                widgets::cell_at(ui, cols[0], CellAlign::Left, |ui| {
                    ui.label(
                        RichText::new(util::format_date_id(&t.date))
                            .size(13.0)
                            .color(theme::text()),
                    );
                });
                // Kategori
                widgets::cell_at(ui, cols[1], CellAlign::Left, |ui| {
                    widgets::dot(ui, util::hex_to_color(&t.color));
                    // `truncate` memotong sesuai lebar kolom yang tersedia,
                    // jadi teks panjang tidak menabrak kolom sebelahnya.
                    ui.add(
                        egui::Label::new(
                            RichText::new(&t.category_name)
                                .size(13.0)
                                .strong()
                                .color(theme::text()),
                        )
                        .truncate(),
                    );
                });
                // Akun
                widgets::cell_at(ui, cols[2], CellAlign::Left, |ui| {
                    let name = t.account_name.clone().unwrap_or_else(|| "Tunai".into());
                    ui.add(
                        egui::Label::new(RichText::new(name).size(13.0).color(theme::text_muted()))
                            .truncate(),
                    );
                });
                // Deskripsi
                widgets::cell_at(ui, cols[3], CellAlign::Left, |ui| {
                    let desc = t
                        .description
                        .clone()
                        .filter(|d| !d.is_empty())
                        .unwrap_or_else(|| "-".into());
                    ui.add(
                        egui::Label::new(RichText::new(desc).size(13.0).color(theme::text_muted()))
                            .truncate(),
                    );
                });
                // Tipe
                widgets::cell_at(ui, cols[4], CellAlign::Left, |ui| {
                    let (bg, fg) = if t.kind == "income" {
                        (theme::income_badge_bg(), theme::income_badge_fg())
                    } else {
                        (theme::expense_badge_bg(), theme::expense_badge_fg())
                    };
                    widgets::badge(ui, util::transaction_type_label(&t.kind), bg, fg);
                });
                // Jumlah
                widgets::cell_at(ui, cols[5], CellAlign::Right, |ui| {
                    let color = if t.kind == "income" {
                        theme::income_text()
                    } else {
                        theme::expense_text()
                    };
                    let sign = if t.kind == "income" { "+" } else { "-" };
                    ui.label(
                        RichText::new(format!("{} {}", sign, util::format_rupiah(t.amount)))
                            .size(13.0)
                            .strong()
                            .color(color),
                    );
                });
                // Aksi
                widgets::cell_at(ui, cols[6], CellAlign::Right, |ui| {
                    // `space-x-3` pada versi PHP (12px), termasuk jarak antar tautan.
                    ui.spacing_mut().item_spacing.x = 12.0;
                    if widgets::link(ui, "Hapus", theme::danger_text(), theme::danger_text_hover()).clicked() {
                        app.confirm = Some(Confirm {
                            title: "Hapus Transaksi".into(),
                            message: "Yakin ingin menghapus transaksi ini?".into(),
                            action: ConfirmAction::DeleteTransaction(row_id),
                        });
                    }
                    if widgets::link(ui, "Edit", theme::primary_text(), theme::primary_text_hover()).clicked() {
                        app.open_tx_edit(row_id);
                    }
                });
            });
        }

        // ---- Paginasi ----
        // Garis pemisah sudah dibuat oleh border bawah baris terakhir tabel.
        if data.total_pages > 1 {
            ui.horizontal(|ui| {
                ui.add_space(widgets::CELL_PAD);
                ui.label(
                    RichText::new(format!(
                        "Halaman {} dari {} \u{00B7} Menampilkan {}-{} dari {} transaksi",
                        data.current_page,
                        data.total_pages,
                        row_start,
                        row_end,
                        data.total_count
                    ))
                    .size(13.0)
                    .color(theme::text_soft()),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(widgets::CELL_PAD);
                    let mut page = data.total_pages;
                    for _ in 0..data.total_pages {
                        let active = page == data.current_page;
                        let style = if active {
                            ButtonStyle {
                                fill: theme::primary_soft(),
                                hover: theme::primary_soft(),
                                text: theme::primary_text(),
                            }
                        } else {
                            ButtonStyle {
                                fill: theme::surface(),
                                hover: theme::secondary_fill(),
                                text: theme::secondary_text(),
                            }
                        };
                        if widgets::styled_button(ui, &page.to_string(), style, Some(40.0)).clicked() && !active
                        {
                            app.tx_page = page;
                        }
                        page = page.saturating_sub(1);
                        ui.add_space(4.0);
                    }
                });
            });
            ui.add_space(10.0);
        }
    });
}

/// Menampilkan form tambah/edit transaksi sebagai modal.
pub fn show_form(app: &mut App, ctx: &egui::Context) {
    let Some(mut form) = app.tx_form.take() else {
        return;
    };

    let mut close = false;
    let mut save = false;

    let size = egui::vec2(560.0, 640.0);
    widgets::modal(ctx, "tx_form_modal", size, |ui| {
        let is_edit = form.id.is_some();

        ui.horizontal(|ui| {
            if widgets::back_arrow(ui).clicked() {
                close = true;
            }
            ui.label(
                RichText::new(if is_edit { "Edit Transaksi" } else { "Tambah Transaksi" })
                    .size(20.0)
                    .strong()
                    .color(theme::text()),
            );
        });
        ui.add_space(16.0);

        // Tipe transaksi
        widgets::field_label(ui, "Tipe Transaksi");
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
                form.category_id = None;
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
                form.category_id = None;
            }
        });
        ui.add_space(14.0);

        // Kategori
        widgets::field_label(ui, "Kategori");
        ui.add_space(4.0);
        let categories: Vec<(i64, String)> = form
            .categories
            .iter()
            .filter(|c| c.kind == form.kind)
            .map(|c| (c.id, c.name.clone()))
            .collect();
        let selected_name = categories
            .iter()
            .find(|(id, _)| Some(*id) == form.category_id)
            .map(|(_, name)| name.clone())
            .unwrap_or_else(|| "Pilih Kategori".to_string());
        let combo_width = ui.available_width();
        widgets::combo(ui, "tx_category", &selected_name, combo_width, |ui| {
            for (id, name) in &categories {
                if ui.selectable_label(Some(*id) == form.category_id, name).clicked() {
                    form.category_id = Some(*id);
                }
            }
        });
        ui.add_space(14.0);

        // Akun
        widgets::field_label(ui, "Akun / Dompet");
        ui.add_space(4.0);
        let accounts: Vec<(i64, String)> = form
            .accounts
            .iter()
            .map(|a| {
                (
                    a.id,
                    format!("{} ({})", a.name, util::account_type_label(&a.kind)),
                )
            })
            .collect();
        let selected_account = accounts
            .iter()
            .find(|(id, _)| Some(*id) == form.account_id)
            .map(|(_, name)| name.clone())
            .unwrap_or_else(|| "Pilih Akun".to_string());
        let combo_width = ui.available_width();
        widgets::combo(ui, "tx_account", &selected_account, combo_width, |ui| {
            for (id, name) in &accounts {
                if ui.selectable_label(Some(*id) == form.account_id, name).clicked() {
                    form.account_id = Some(*id);
                }
            }
        });
        ui.add_space(14.0);

        // Jumlah
        widgets::field_label(ui, "Jumlah (Rp)");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.amount, "Contoh: 50000", field_width);
        ui.add_space(14.0);

        // Tanggal
        widgets::field_label(ui, "Tanggal");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.date, "YYYY-MM-DD", field_width);
        ui.add_space(14.0);

        // Deskripsi
        widgets::field_label(ui, "Deskripsi (Opsional)");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_area(ui, &mut form.description, "Tambahkan catatan\u{2026}", 3, field_width);
        ui.add_space(16.0);

        if let Some(err) = &form.error {
            widgets::error_box(ui, err);
            ui.add_space(12.0);
        }

        // Tombol
        let gap = 12.0;
        let total = ui.available_width();
        let primary_w = total * 0.62;
        let secondary_w = total - primary_w - gap;
        let submit_label = if is_edit {
            "Simpan Perubahan"
        } else {
            "Tambah Transaksi"
        };
        ui.horizontal(|ui| {
            if widgets::styled_button(ui, submit_label, ButtonStyle::primary(), Some(primary_w)).clicked()
            {
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
        match submit(&form, app) {
            Ok(()) => {
                app.notify_success(if form.id.is_some() {
                    "Perubahan transaksi berhasil disimpan"
                } else {
                    "Transaksi berhasil ditambahkan"
                });
            }
            Err(message) => {
                form.error = Some(message);
                app.tx_form = Some(form);
            }
        }
        return;
    }

    if !close {
        app.tx_form = Some(form);
    }
}

/// Menyimpan form (tambah atau ubah) ke database.
fn submit(form: &TxForm, app: &App) -> Result<(), String> {
    let amount = util::parse_amount(&form.amount)
        .ok_or_else(|| "Jumlah tidak valid".to_string())?;
    let category_id = form
        .category_id
        .ok_or_else(|| "Kategori wajib dipilih".to_string())?;
    let account_id = form.account_id.unwrap_or(1);
    let description = {
        let d = form.description.trim();
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    };

    let conn = app.db.conn();
    match form.id {
        Some(id) => TransactionController::update(
            conn,
            id,
            category_id,
            account_id,
            amount,
            description,
            form.date.trim(),
        ),
        None => TransactionController::store(
            conn,
            category_id,
            account_id,
            amount,
            &form.kind,
            description,
            form.date.trim(),
        )
        .map(|_| ()),
    }
}
