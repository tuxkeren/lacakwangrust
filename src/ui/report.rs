//! Dialog pemilihan periode dan pembuatan laporan PDF dua kolom.
//!
//! Laporan berisi pemasukan di kolom kiri, pengeluaran di kolom kanan,
//! lengkap dengan tanggal, deskripsi, jumlah, dan sisa saldo di akhir.
//! Lokasi & nama berkas dipilih lewat dialog "Simpan Sebagai" bawaan
//! sistem, dengan folder serta nama default sebagai titik awal.

use std::path::{Path, PathBuf};

use eframe::egui::{self, Align, Layout, RichText, Vec2};

use crate::config;
use crate::controllers::ReportController;
use crate::models::ReportPeriod;
use crate::pdf::{self, ReportMeta};

use super::theme;
use super::widgets::{self, ButtonStyle};
use super::App;

pub fn show(app: &mut App, ctx: &egui::Context) {
    if !app.report_open {
        return;
    }

    let mut close = false;
    let mut save_as = false;
    let period = app.report_period;
    let report_dir = config::resolve_report_dir();
    let report_dir_display = report_dir.display().to_string();

    widgets::modal(ctx, "report_modal", Vec2::new(560.0, 0.0), |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Laporan PDF")
                    .size(18.0)
                    .strong()
                    .color(theme::text()),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if widgets::link(
                    ui,
                    "\u{00D7}",
                    theme::text_muted(),
                    theme::text_soft(),
                )
                .clicked()
                {
                    close = true;
                }
            });
        });

        ui.add_space(4.0);
        ui.label(
            RichText::new("Pilih periode laporan dan buat berkas PDF dua kolom (pemasukan & pengeluaran).")
                .size(13.0)
                .color(theme::text_muted()),
        );

        ui.add_space(16.0);
        ui.label(
            RichText::new("Periode laporan")
                .size(13.0)
                .strong()
                .color(theme::text_soft()),
        );
        ui.add_space(8.0);

        let gap = 10.0;
        let available = ui.available_width();
        let card_w = (available - gap * 2.0) / 3.0;
        ui.horizontal(|ui| {
            let mut p = period;
            if widgets::choice_card(
                ui,
                ReportPeriod::All.label(),
                Some("Semua transaksi yang tercatat"),
                period == ReportPeriod::All,
                card_w,
                theme::primary_text(),
                theme::primary_soft(),
            )
            .clicked()
            {
                p = ReportPeriod::All;
            }
            ui.add_space(gap);
            if widgets::choice_card(
                ui,
                ReportPeriod::ThisMonth.label(),
                Some("Transaksi bulan berjalan"),
                period == ReportPeriod::ThisMonth,
                card_w,
                theme::primary_text(),
                theme::primary_soft(),
            )
            .clicked()
            {
                p = ReportPeriod::ThisMonth;
            }
            ui.add_space(gap);
            if widgets::choice_card(
                ui,
                ReportPeriod::ThisYear.label(),
                Some("Transaksi tahun berjalan"),
                period == ReportPeriod::ThisYear,
                card_w,
                theme::primary_text(),
                theme::primary_soft(),
            )
            .clicked()
            {
                p = ReportPeriod::ThisYear;
            }
            app.report_period = p;
        });

        ui.add_space(16.0);
        ui.label(
            RichText::new("Lokasi penyimpanan")
                .size(13.0)
                .strong()
                .color(theme::text_soft()),
        );
        ui.add_space(6.0);
        ui.label(
            RichText::new(format!(
                "Folder default: {} (jadi titik awal dialog sistem)",
                report_dir_display
            ))
            .size(13.0)
            .color(theme::text_muted()),
        );

        if let Some(err) = &app.report_error {
            ui.add_space(12.0);
            widgets::error_box(ui, err);
        }

        ui.add_space(20.0);
        let full = ui.available_width();
        ui.horizontal(|ui| {
            let left = (full - gap) * 0.6;
            let right = full - gap - left;
            if widgets::styled_button(
                ui,
                "\u{1F9FE} Simpan Sebagai…",
                ButtonStyle::primary(),
                Some(left),
            )
            .clicked()
            {
                save_as = true;
            }
            ui.add_space(gap);
            if widgets::styled_button(ui, "Batal", ButtonStyle::secondary(), Some(right))
                .clicked()
            {
                close = true;
            }
        });
    });

    if save_as {
        // Data dibangun dulu agar galat (mis. tidak ada transaksi) muncul
        // sebelum dialog sistem terbuka.
        let conn = app.db.conn();
        match ReportController::build(conn, app.report_period) {
            Ok(data) => {
                // `None` berarti pengguna membatalkan dialog sistem:
                // biarkan dialog laporan tetap terbuka tanpa pesan galat.
                if let Some(target) = pilih_lokasi_simpan(&report_dir) {
                    let meta = ReportMeta {
                        app_name: app.config.app_name.clone(),
                        version: app.config.version.clone(),
                        currency_code: app.config.currency_code.clone(),
                        author: app.config.author.clone(),
                    };
                    match pdf::write_laporan(&target, &data, &meta) {
                        Ok(_) => {
                            app.report_error = None;
                            app.notify_success(format!(
                                "Laporan PDF tersimpan: {}",
                                target.display()
                            ));
                            close = true;
                        }
                        Err(e) => {
                            app.report_error = Some(e);
                        }
                    }
                }
            }
            Err(e) => {
                app.report_error = Some(e.to_string());
            }
        }
    }

    if close || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.report_open = false;
        app.report_error = None;
    }
}

/// Membuka dialog "Simpan Sebagai" bawaan sistem untuk laporan PDF.
///
/// Mengembalikan `None` bila pengguna membatalkan dialog. Nama berkas
/// diisi awal dengan nama default laporan; folder default dipakai sebagai
/// titik awal bila folder tersebut ada.
fn pilih_lokasi_simpan(report_dir: &Path) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new()
        .set_title("Simpan Laporan PDF Sebagai")
        .add_filter("Dokumen PDF", &["pdf"])
        .set_file_name(pdf::default_report_file_name());
    if report_dir.is_dir() {
        dialog = dialog.set_directory(report_dir);
    }

    let mut path = dialog.save_file()?;
    // Pastikan ekstensi `.pdf` ikut tertulis bila pengguna tidak mengetiknya
    // (mis. mengetik `laporan-20261018` saja, atau nama berakhir titik).
    if path.extension().map_or(true, |e| e.is_empty()) {
        path.set_extension("pdf");
    }
    Some(path)
}