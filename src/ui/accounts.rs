//! Halaman Manajemen Akun / Dompet — padanan dari `views/accounts/index.php`.

use eframe::egui::{self, Align, Layout, RichText, Stroke};

use crate::controllers::AccountController;
use crate::models::AccountSummary;
use crate::util;

use super::app::{AccForm, App, Confirm, ConfirmAction};
use super::widgets::{self, ButtonStyle};
use super::theme;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let data = match AccountController::index(app.db.conn()) {
        Ok(d) => d,
        Err(e) => {
            widgets::error_box(ui, &format!("Gagal memuat akun: {e}"));
            return;
        }
    };

    // ---- Header ----
    ui.horizontal(|ui| {
        widgets::page_title(ui, "Manajemen Akun / Dompet");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if widgets::styled_button(ui, "+ Tambah Akun", ButtonStyle::primary(), None).clicked() {
                app.acc_form = Some(new_form());
            }
        });
    });
    ui.add_space(18.0);

    // ---- Kartu ringkasan ----
    ui.columns(3, |cols| {
        widgets::stat_card(
            &mut cols[0],
            "Total Pemasukan",
            &util::format_rupiah(data.total_income),
            theme::GRAY_900,
            "Dari semua akun",
            theme::GREEN_500,
            theme::GREEN_100,
            theme::GREEN_600,
            "\u{2B06}",
        );
        widgets::stat_card(
            &mut cols[1],
            "Total Pengeluaran",
            &util::format_rupiah(data.total_expense),
            theme::GRAY_900,
            "Dari semua akun",
            theme::RED_500,
            theme::RED_100,
            theme::RED_600,
            "\u{2B07}",
        );
        let balance_color = if data.total_balance >= 0.0 {
            theme::GREEN_600
        } else {
            theme::RED_600
        };
        widgets::stat_card(
            &mut cols[2],
            "Total Saldo",
            &util::format_rupiah(data.total_balance),
            balance_color,
            "Gabungan semua akun",
            theme::BLUE_500,
            theme::BLUE_100,
            theme::BLUE_600,
            "\u{1F4B0}",
        );
    });
    ui.add_space(24.0);

    // ---- Breakdown per akun ----
    ui.columns(2, |cols| {
        account_breakdown(
            &mut cols[0],
            "Pemasukan per Akun",
            &data.accounts,
            data.total_income,
            theme::GREEN_500,
            "Belum ada pemasukan",
            true,
        );
        account_breakdown(
            &mut cols[1],
            "Pengeluaran per Akun",
            &data.accounts,
            data.total_expense,
            theme::RED_500,
            "Belum ada pengeluaran",
            false,
        );
    });
    ui.add_space(24.0);

    // ---- Daftar akun ----
    widgets::card(ui, |ui| {
        ui.horizontal(|ui| {
            widgets::section_title(ui, "Daftar Akun");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                widgets::muted(ui, &format!("{} akun aktif", data.accounts.len()));
            });
        });
        ui.add_space(6.0);

        if data.accounts.is_empty() {
            if widgets::empty_state(
                ui,
                "\u{1F4CA}",
                "Belum ada akun",
                "Mulai dengan menambahkan akun/dompet pertama Anda.",
                Some("Tambah Akun Pertama"),
            ) {
                app.acc_form = Some(new_form());
            }
            return;
        }

        for (idx, acc) in data.accounts.iter().enumerate() {
            if idx > 0 {
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.0),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, theme::GRAY_200));
            }
            ui.add_space(12.0);
            account_row(ui, acc, app);
            ui.add_space(12.0);
        }
    });
}

fn new_form() -> AccForm {
    AccForm {
        id: None,
        name: String::new(),
        kind: "bank".to_string(),
        balance: String::new(),
        error: None,
    }
}

fn account_breakdown(
    ui: &mut egui::Ui,
    title: &str,
    accounts: &[AccountSummary],
    total: f64,
    bar_color: egui::Color32,
    empty_text: &str,
    use_income: bool,
) {
    widgets::card(ui, |ui| {
        widgets::section_title(ui, title);
        ui.add_space(12.0);

        let has_data = accounts
            .iter()
            .any(|a| if use_income { a.total_income > 0.0 } else { a.total_expense > 0.0 });
        if !has_data || total <= 0.0 {
            widgets::muted(ui, empty_text);
            return;
        }

        for acc in accounts {
            let value = if use_income {
                acc.total_income
            } else {
                acc.total_expense
            };
            if value <= 0.0 {
                continue;
            }

            ui.horizontal(|ui| {
                widgets::dot(ui, util::account_type_color(&acc.kind));
                ui.label(
                    RichText::new(&acc.name)
                        .size(13.0)
                        .strong()
                        .color(theme::GRAY_700),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(util::format_rupiah(value))
                            .size(13.0)
                            .strong()
                            .color(theme::GRAY_900),
                    );
                });
            });
            ui.add_space(4.0);
            widgets::progress_bar(ui, (value / total) as f32, bar_color);
            ui.add_space(12.0);
        }
    });
}

fn account_row(ui: &mut egui::Ui, acc: &AccountSummary, app: &mut App) {
    let type_color = util::account_type_color(&acc.kind);
    let (badge_bg, badge_fg) = match acc.kind.as_str() {
        "cash" => (theme::GREEN_100, theme::GREEN_800),
        "bank" => (theme::BLUE_100, theme::BLUE_800),
        "ewallet" => (theme::PURPLE_100, theme::PURPLE_800),
        _ => (theme::GRAY_100, theme::GRAY_800),
    };

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                widgets::dot(ui, type_color);
                ui.label(
                    RichText::new(&acc.name)
                        .size(15.0)
                        .strong()
                        .color(theme::GRAY_900),
                );
                widgets::badge(ui, util::account_type_label(&acc.kind), badge_bg, badge_fg);
            });
            ui.label(
                RichText::new(format!(
                    "Saldo awal {} \u{00B7} Masuk +{} \u{00B7} Keluar -{}",
                    util::format_rupiah(acc.initial_balance),
                    util::format_rupiah(acc.total_income),
                    util::format_rupiah(acc.total_expense),
                ))
                .size(13.0)
                .color(theme::GRAY_600),
            );
        });

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.vertical(|ui| {
                let color = if acc.current_balance >= 0.0 {
                    theme::GREEN_600
                } else {
                    theme::RED_600
                };
                let sign = if acc.current_balance >= 0.0 { "+" } else { "" };
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{}{}",
                            sign,
                            util::format_rupiah(acc.current_balance)
                        ))
                        .size(18.0)
                        .strong()
                        .color(color),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if widgets::link(ui, "Hapus", theme::RED_600, theme::RED_800).clicked() {
                        app.confirm = Some(Confirm {
                            title: "Hapus Akun".into(),
                            message: format!(
                                "Yakin ingin menghapus akun \"{}\"? Transaksi akan tetap ada namun tanpa akun.",
                                acc.name
                            ),
                            action: ConfirmAction::DeleteAccount(acc.id),
                        });
                    }
                    ui.add_space(12.0);
                    if widgets::link(ui, "Edit", theme::PRIMARY, theme::BLUE_900).clicked() {
                        app.open_acc_edit(acc.id);
                    }
                });
            });
        });
    });
}

/// Membuka modal form akun.
pub fn show_form(app: &mut App, ctx: &egui::Context) {
    let Some(mut form) = app.acc_form.take() else {
        return;
    };

    let mut close = false;
    let mut save = false;

    widgets::modal(ctx, "acc_form_modal", egui::vec2(470.0, 440.0), |ui| {
        let is_edit = form.id.is_some();
        ui.label(
            RichText::new(if is_edit { "Edit Akun" } else { "Tambah Akun" })
                .size(20.0)
                .strong()
                .color(theme::GRAY_900),
        );
        ui.add_space(16.0);

        // Nama akun
        widgets::field_label(ui, "Nama Akun");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.name, "Contoh: Bank BCA, Dompet Utama", field_width);
        ui.add_space(14.0);

        // Tipe akun
        widgets::field_label(ui, "Tipe Akun");
        ui.add_space(4.0);
        let types = [
            ("cash", "Tunai / Cash"),
            ("bank", "Bank"),
            ("ewallet", "E-Wallet"),
            ("other", "Lainnya"),
        ];
        let selected = types
            .iter()
            .find(|(k, _)| *k == form.kind)
            .map(|(k, label)| format!("{} {}", util::account_type_icon(k), label))
            .unwrap_or_else(|| "Pilih Tipe".to_string());
        let combo_width = ui.available_width();
        widgets::combo(ui, "acc_type", &selected, combo_width, |ui| {
            for (k, label) in types {
                let text = format!("{} {}", util::account_type_icon(k), label);
                if ui.selectable_label(form.kind == k, text).clicked() {
                    form.kind = k.to_string();
                }
            }
        });
        ui.add_space(14.0);

        // Saldo awal
        widgets::field_label(ui, "Saldo Awal (Opsional)");
        ui.add_space(4.0);
        let field_width = ui.available_width();
        widgets::text_input(ui, &mut form.balance, "Contoh: 1000000", field_width);
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
            let label = if is_edit { "Simpan Perubahan" } else { "Simpan" };
            if widgets::styled_button(ui, label, ButtonStyle::primary(), Some(primary_w)).clicked() {
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
        let balance = if form.balance.trim().is_empty() {
            0.0
        } else {
            match util::parse_amount(&form.balance) {
                Some(v) => v,
                None => {
                    form.error = Some("Saldo awal tidak valid".into());
                    app.acc_form = Some(form);
                    return;
                }
            }
        };

        match AccountController::save(app.db.conn(), form.id, &form.name, &form.kind, balance) {
            Ok(()) => {
                app.notify_success(if form.id.is_some() {
                    "Akun berhasil diperbarui"
                } else {
                    "Akun berhasil ditambahkan"
                });
            }
            Err(message) => {
                form.error = Some(message);
                app.acc_form = Some(form);
            }
        }
        return;
    }

    if !close {
        app.acc_form = Some(form);
    }
}
