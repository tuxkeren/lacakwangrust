//! Halaman Dashboard — padanan dari `views/dashboard/index.php`.

use eframe::egui::{self, Align, Layout, RichText, Stroke};

use crate::controllers::DashboardController;
use crate::models::transaction::CategoryBreakdown;
use crate::util;

use super::app::{App, Page};
use super::{theme, widgets};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let data = match DashboardController::index(app.db.conn()) {
        Ok(d) => d,
        Err(e) => {
            widgets::error_box(ui, &format!("Gagal memuat dashboard: {e}"));
            return;
        }
    };

    widgets::page_title(ui, "Dashboard");
    ui.add_space(18.0);

    // ---- Kartu ringkasan ----
    ui.columns(3, |cols| {
        widgets::stat_card(
            &mut cols[0],
            "Total Pemasukan",
            &util::format_rupiah(data.summary.income),
            theme::text(),
            "30 hari terakhir",
            theme::income_accent(),
            theme::income_badge_bg(),
            theme::income_text(),
            "\u{2B06}",
        );
        widgets::stat_card(
            &mut cols[1],
            "Total Pengeluaran",
            &util::format_rupiah(data.summary.expense),
            theme::text(),
            "30 hari terakhir",
            theme::expense_accent(),
            theme::expense_badge_bg(),
            theme::expense_text(),
            "\u{2B07}",
        );
        let balance_color = if data.balance >= 0.0 {
            theme::income_text()
        } else {
            theme::expense_text()
        };
        widgets::stat_card(
            &mut cols[2],
            "Saldo",
            &util::format_rupiah(data.balance),
            balance_color,
            "Selisih 30 hari",
            theme::primary_fill(),
            theme::primary_badge_bg(),
            theme::primary_badge_fg(),
            "\u{1F4B0}",
        );
    });

    ui.add_space(24.0);

    // ---- Breakdown per kategori ----
    ui.columns(2, |cols| {
        breakdown_card(
            &mut cols[0],
            "Pemasukan per Kategori",
            &data.income_breakdown,
            data.summary.income,
            "Belum ada data pemasukan",
        );
        breakdown_card(
            &mut cols[1],
            "Pengeluaran per Kategori",
            &data.expense_breakdown,
            data.summary.expense,
            "Belum ada data pengeluaran",
        );
    });

    ui.add_space(24.0);

    // ---- Transaksi terbaru ----
    widgets::card(ui, |ui| {
        ui.horizontal(|ui| {
            widgets::section_title(ui, "Transaksi Terbaru");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if widgets::link(ui, "Lihat Semua \u{00BB}", theme::primary_text(), theme::primary_text_hover())
                    .clicked()
                {
                    app.page = Page::Transactions;
                }
            });
        });
        ui.add_space(6.0);

        if data.recent_transactions.is_empty() {
            if widgets::empty_state(
                ui,
                "\u{1F4C4}",
                "Belum ada transaksi",
                "Mulai catat pemasukan atau pengeluaran pertama Anda.",
                Some("+ Tambah Transaksi Pertama"),
            ) {
                app.open_tx_form();
            }
        } else {
            for (idx, t) in data.recent_transactions.iter().enumerate() {
                if idx > 0 {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 1.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().hline(
                        rect.x_range(),
                        rect.center().y,
                        Stroke::new(1.0, theme::border()),
                    );
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            widgets::dot(ui, util::hex_to_color(&t.color));
                            ui.label(
                                RichText::new(&t.category_name)
                                    .size(14.0)
                                    .strong()
                                    .color(theme::text()),
                            );
                            let (bg, fg) = if t.kind == "income" {
                                (theme::income_badge_bg(), theme::income_badge_fg())
                            } else {
                                (theme::expense_badge_bg(), theme::expense_badge_fg())
                            };
                            widgets::badge(ui, util::transaction_type_label(&t.kind), bg, fg);
                        });
                        if let Some(desc) = t.description.as_deref() {
                            if !desc.is_empty() {
                                ui.label(RichText::new(desc).size(13.0).color(theme::text_muted()));
                            }
                        }
                        ui.label(
                            RichText::new(util::format_date_id(&t.date))
                                .size(12.0)
                                .color(theme::text_muted()),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let color = if t.kind == "income" {
                            theme::income_text()
                        } else {
                            theme::expense_text()
                        };
                        let sign = if t.kind == "income" { "+" } else { "-" };
                        ui.label(
                            RichText::new(format!("{} {}", sign, util::format_rupiah(t.amount)))
                                .size(18.0)
                                .strong()
                                .color(color),
                        );
                    });
                });
                ui.add_space(12.0);
            }
        }
    });
}

/// Kartu daftar kategori dengan progress bar.
fn breakdown_card(
    ui: &mut egui::Ui,
    title: &str,
    items: &[CategoryBreakdown],
    total: f64,
    empty_text: &str,
) {
    widgets::card(ui, |ui| {
        widgets::section_title(ui, title);
        ui.add_space(12.0);

        if items.is_empty() {
            widgets::muted(ui, empty_text);
            return;
        }

        for cat in items {
            let color = util::hex_to_color(&cat.color);
            ui.horizontal(|ui| {
                widgets::dot(ui, color);
                ui.label(
                    RichText::new(&cat.name)
                        .size(13.0)
                        .strong()
                        .color(theme::text_soft()),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(util::format_rupiah(cat.total))
                            .size(13.0)
                            .strong()
                            .color(theme::text()),
                    );
                });
            });
            ui.add_space(4.0);
            let fraction = if total > 0.0 { cat.total / total } else { 0.0 };
            widgets::progress_bar(ui, fraction as f32, color);
            ui.add_space(12.0);
        }
    });
}
