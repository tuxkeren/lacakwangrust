//! Struktur aplikasi utama: navigasi, kerangka halaman, modal, dan notifikasi.
//!
//! Padanan dari `views/layouts/header.php` + `footer.php` dan `public/index.php`.

use eframe::egui::{self, Align, Color32, Frame, Layout, Margin, RichText, Stroke};
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::controllers::{AccountController, CategoryController, TransactionController, TransactionFilter};
use crate::db::Database;
use crate::models::{Account, Category};

use super::{accounts, categories, dashboard, theme, transactions, widgets};
use widgets::ButtonStyle;

/// Halaman yang sedang aktif.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Transactions,
    Categories,
    Accounts,
}

/// Notifikasi sederhana (pengganti alert PHP).
#[derive(Clone, Debug)]
pub struct Toast {
    pub message: String,
    pub success: bool,
}

/// Aksi yang perlu konfirmasi sebelum dijalankan.
#[derive(Clone, Debug)]
pub enum ConfirmAction {
    DeleteTransaction(i64),
    DeleteCategory(i64),
    DeleteAccount(i64),
}

/// Dialog konfirmasi.
#[derive(Clone, Debug)]
pub struct Confirm {
    pub title: String,
    pub message: String,
    pub action: ConfirmAction,
}

/// Form transaksi (tambah/edit).
#[derive(Clone, Debug)]
pub struct TxForm {
    pub id: Option<i64>,
    pub kind: String,
    pub category_id: Option<i64>,
    pub account_id: Option<i64>,
    pub amount: String,
    pub date: String,
    pub description: String,
    pub error: Option<String>,
    pub categories: Vec<Category>,
    pub accounts: Vec<Account>,
}

/// Form kategori (tambah/edit).
#[derive(Clone, Debug)]
pub struct CatForm {
    pub id: Option<i64>,
    pub name: String,
    pub color: String,
    pub kind: String,
    pub error: Option<String>,
}

/// Form akun (tambah/edit).
#[derive(Clone, Debug)]
pub struct AccForm {
    pub id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub balance: String,
    pub error: Option<String>,
}

/// State global aplikasi.
pub struct App {
    pub db: Database,
    pub config: Config,
    pub page: Page,
    pub toast: Option<Toast>,
    toast_at: Option<Instant>,

    // Daftar transaksi
    pub tx_filter: TransactionFilter,
    pub tx_page: usize,

    // Form-form
    pub tx_form: Option<TxForm>,
    pub cat_form: Option<CatForm>,
    pub acc_form: Option<AccForm>,
    pub confirm: Option<Confirm>,
    /// Dialog "Tentang" (info aplikasi + kredit pembuat) sedang terbuka.
    pub about_open: bool,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        config: Config,
        db: Database,
    ) -> Self {
        theme::configure(&cc.egui_ctx);

        Self {
            db,
            config,
            page: Page::Dashboard,
            toast: None,
            toast_at: None,
            tx_filter: TransactionFilter::All,
            tx_page: 1,
            tx_form: None,
            cat_form: None,
            acc_form: None,
            confirm: None,
            about_open: false,
        }
    }

    /// Menampilkan notifikasi sukses.
    pub fn notify_success(&mut self, message: impl Into<String>) {
        self.toast = Some(Toast {
            message: message.into(),
            success: true,
        });
        self.toast_at = Some(Instant::now());
    }

    /// Menampilkan notifikasi gagal.
    pub fn notify_error(&mut self, message: impl Into<String>) {
        self.toast = Some(Toast {
            message: message.into(),
            success: false,
        });
        self.toast_at = Some(Instant::now());
    }

    /// Membuka form transaksi baru.
    pub fn open_tx_form(&mut self) {
        match TransactionController::create_data(self.db.conn()) {
            Ok((categories, accounts)) => {
                let account_id = accounts.first().map(|a| a.id);
                self.tx_form = Some(TxForm {
                    id: None,
                    kind: "income".to_string(),
                    category_id: None,
                    account_id,
                    amount: String::new(),
                    date: crate::util::today_iso(),
                    description: String::new(),
                    error: None,
                    categories,
                    accounts,
                });
            }
            Err(e) => self.notify_error(format!("Gagal memuat data: {e}")),
        }
    }

    /// Membuka form kategori untuk mengubah data yang ada.
    pub fn open_cat_edit(&mut self, id: i64) {
        match CategoryController::detail(self.db.conn(), id) {
            Ok(Some(cat)) => {
                self.cat_form = Some(CatForm {
                    id: Some(cat.id),
                    name: cat.name,
                    color: cat.color,
                    kind: cat.kind,
                    error: None,
                });
            }
            Ok(None) => self.notify_error("Kategori tidak ditemukan"),
            Err(e) => self.notify_error(format!("Gagal memuat data: {e}")),
        }
    }

    /// Membuka form akun untuk mengubah data yang ada (data diambil ulang
    /// dari database agar form selalu menampilkan nilai terbaru).
    pub fn open_acc_edit(&mut self, id: i64) {
        match AccountController::detail(self.db.conn(), id) {
            Ok(Some(acc)) => {
                self.acc_form = Some(AccForm {
                    id: Some(acc.id),
                    name: acc.name,
                    kind: acc.kind,
                    balance: format!("{}", acc.balance),
                    error: None,
                });
            }
            Ok(None) => self.notify_error("Akun tidak ditemukan"),
            Err(e) => self.notify_error(format!("Gagal memuat data: {e}")),
        }
    }

    /// Membuka form transaksi untuk mengubah data yang ada.
    pub fn open_tx_edit(&mut self, id: i64) {
        match TransactionController::edit_data(self.db.conn(), id) {
            Ok(Some((detail, categories, accounts))) => {
                self.tx_form = Some(TxForm {
                    id: Some(detail.id),
                    kind: detail.kind.clone(),
                    category_id: Some(detail.category_id),
                    account_id: detail.account_id,
                    amount: format!("{}", detail.amount),
                    date: detail.date.clone(),
                    description: detail.description.clone().unwrap_or_default(),
                    error: None,
                    categories,
                    accounts,
                });
            }
            Ok(None) => self.notify_error("Transaksi tidak ditemukan"),
            Err(e) => self.notify_error(format!("Gagal memuat data: {e}")),
        }
    }

    // ---------------------------------------------------------------
    // Kerangka halaman
    // ---------------------------------------------------------------

    fn navbar(&self, ui: &mut egui::Ui) -> (Option<Page>, bool) {
        let mut target = None;
        let mut about = false;

        egui::Panel::top("navbar")
            .show_separator_line(false)
            .frame(
                Frame::NONE
                    .fill(Color32::WHITE)
                    .inner_margin(Margin {
                        left: 32,
                        right: 32,
                        top: 14,
                        bottom: 14,
                    })
                    .stroke(Stroke::new(1.0, theme::GRAY_200)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("\u{1F4B0} {}", self.config.app_name))
                            .size(22.0)
                            .strong()
                            .color(theme::PRIMARY),
                    );

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Ditambahkan lebih dulu supaya tampil paling kanan,
                        // setelah menu halaman.
                        if widgets::nav_button(ui, "Tentang", false).clicked() {
                            about = true;
                        }
                        ui.add_space(6.0);

                        for (page, label) in [
                            (Page::Accounts, "Akun"),
                            (Page::Categories, "Kategori"),
                            (Page::Transactions, "Transaksi"),
                            (Page::Dashboard, "Dashboard"),
                        ] {
                            if widgets::nav_button(ui, label, self.page == page).clicked() {
                                target = Some(page);
                            }
                            ui.add_space(6.0);
                        }
                    });
                });
            });

        (target, about)
    }

    fn footer(&self, ui: &mut egui::Ui) {
        let year = chrono::Local::now().format("%Y");
        egui::Panel::bottom("footer")
            .show_separator_line(false)
            .frame(
                Frame::NONE
                    .fill(Color32::WHITE)
                    .inner_margin(Margin {
                        left: 32,
                        right: 32,
                        top: 16,
                        bottom: 16,
                    })
                    .stroke(Stroke::new(1.0, theme::GRAY_200)),
            )
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} v{} - Pencatatan Keuangan Sederhana \u{00A9} {} \u{00B7} Mata uang {}",
                            self.config.app_name,
                            self.config.version,
                            year,
                            self.config.currency_code
                        ))
                        .size(13.0)
                        .color(theme::GRAY_500),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!(
                            "Dibuat oleh {} \u{00B7} {}",
                            self.config.author_credit(),
                            self.config.license
                        ))
                        .size(12.0)
                        .color(theme::GRAY_400),
                    );
                });
            });
    }

    /// Dialog "Tentang": info aplikasi, kredit pembuat, lisensi, dan lokasi
    /// berkas basis data. Dibuka lewat menu "Tentang" pada navbar.
    fn show_about(&mut self, ctx: &egui::Context) {
        if !self.about_open {
            return;
        }

        fn label_cell(ui: &mut egui::Ui, text: &str) {
            ui.label(RichText::new(text).size(13.0).color(theme::GRAY_500));
        }
        fn value_cell(ui: &mut egui::Ui, text: &str) {
            ui.label(RichText::new(text).size(13.0).color(theme::GRAY_800));
        }

        let mut close = false;
        let db_path = crate::config::resolve_db_path();

        widgets::modal(ctx, "about_modal", egui::vec2(520.0, 0.0), |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Tentang Aplikasi")
                        .size(18.0)
                        .strong()
                        .color(theme::GRAY_900),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if widgets::link(ui, "\u{00D7}", theme::GRAY_500, theme::GRAY_700).clicked() {
                        close = true;
                    }
                });
            });

            ui.add_space(2.0);
            ui.label(
                RichText::new("Versi native (desktop) dari aplikasi PHP OOP PelacakDuit.")
                    .size(13.0)
                    .color(theme::GRAY_600),
            );

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            egui::Grid::new("about_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    label_cell(ui, "Aplikasi");
                    value_cell(
                        ui,
                        &format!("{} v{}", self.config.app_name, self.config.version),
                    );
                    ui.end_row();

                    label_cell(ui, "Teknologi");
                    value_cell(ui, "Rust + egui/eframe + SQLite");
                    ui.end_row();

                    label_cell(ui, "Pembuat");
                    value_cell(ui, &self.config.author_credit());
                    ui.end_row();

                    label_cell(ui, "Repositori");
                    ui.hyperlink_to(
                        RichText::new(&self.config.repo_url).size(13.0),
                        &self.config.repo_url,
                    );
                    ui.end_row();

                    label_cell(ui, "Lisensi");
                    value_cell(ui, &self.config.license);
                    ui.end_row();

                    label_cell(ui, "Basis data");
                    ui.add(
                        egui::Label::new(
                            RichText::new(db_path.display().to_string())
                                .size(13.0)
                                .color(theme::GRAY_600),
                        )
                        .truncate(),
                    );
                    ui.end_row();
                });

            ui.add_space(20.0);
            let full = ui.available_width();
            if widgets::styled_button(ui, "Tutup", ButtonStyle::secondary(), Some(full)).clicked() {
                close = true;
            }
        });

        if close || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.about_open = false;
        }
    }

    fn alert_banner(&mut self, ui: &mut egui::Ui) {
        let Some(toast) = self.toast.clone() else {
            return;
        };

        let (bg, fg, border) = if toast.success {
            (theme::GREEN_50, theme::GREEN_800, theme::GREEN_500)
        } else {
            (theme::RED_50, theme::RED_800, theme::RED_500)
        };

        let mut dismiss = false;
        Frame::new()
            .fill(bg)
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(Margin::symmetric(16, 12))
            .stroke(Stroke::new(1.0, border))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&toast.message).size(14.0).color(fg));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if widgets::link(ui, "\u{00D7}", fg, fg).clicked() {
                            dismiss = true;
                        }
                    });
                });
            });
        ui.add_space(16.0);

        if dismiss {
            self.toast = None;
            self.toast_at = None;
        }
    }

    fn show_confirm(&mut self, ctx: &egui::Context) {
        let Some(confirm) = self.confirm.clone() else {
            return;
        };

        let mut decision: Option<bool> = None;
        widgets::modal(
            ctx,
            "confirm_modal",
            egui::vec2(440.0, 220.0),
            |ui| {
                ui.label(
                    RichText::new(&confirm.title)
                        .size(18.0)
                        .strong()
                        .color(theme::GRAY_900),
                );
                ui.add_space(10.0);
                ui.label(
                    RichText::new(&confirm.message)
                        .size(14.0)
                        .color(theme::GRAY_600),
                );
                ui.add_space(20.0);

                let gap = 12.0;
                let w = (ui.available_width() - gap) / 2.0;
                ui.horizontal(|ui| {
                    if widgets::styled_button(ui, "Ya, Hapus", ButtonStyle::danger(), Some(w)).clicked()
                    {
                        decision = Some(true);
                    }
                    ui.add_space(gap);
                    if widgets::styled_button(ui, "Batal", ButtonStyle::secondary(), Some(w)).clicked()
                    {
                        decision = Some(false);
                    }
                });
            },
        );

        match decision {
            Some(true) => {
                let action = confirm.action.clone();
                self.confirm = None;
                self.run_confirm(action);
            }
            Some(false) => self.confirm = None,
            None => {}
        }
    }

    fn run_confirm(&mut self, action: ConfirmAction) {
        let conn = self.db.conn();
        match action {
            ConfirmAction::DeleteTransaction(id) => match TransactionController::delete(conn, id) {
                Ok(()) => {
                    self.notify_success("Transaksi berhasil dihapus");
                }
                Err(e) => self.notify_error(e),
            },
            ConfirmAction::DeleteCategory(id) => match CategoryController::delete(conn, id) {
                Ok(()) => self.notify_success("Kategori berhasil dihapus"),
                Err(e) => self.notify_error(e),
            },
            ConfirmAction::DeleteAccount(id) => match AccountController::delete(conn, id) {
                Ok(()) => self.notify_success("Akun berhasil dihapus"),
                Err(e) => self.notify_error(e),
            },
        }
    }

    fn purge_expired_toast(&mut self) {
        if let Some(at) = self.toast_at {
            if at.elapsed() > Duration::from_secs(5) {
                self.toast = None;
                self.toast_at = None;
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        let (nav_to, about_clicked) = self.navbar(ui);
        self.footer(ui);

        if about_clicked {
            self.about_open = true;
        }

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(theme::GRAY_50).inner_margin(Margin {
                left: 32,
                right: 32,
                top: 28,
                bottom: 28,
            }))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.alert_banner(ui);
                        match self.page {
                            Page::Dashboard => dashboard::show(self, ui),
                            Page::Transactions => transactions::show(self, ui),
                            Page::Categories => categories::show(self, ui),
                            Page::Accounts => accounts::show(self, ui),
                        }
                        ui.add_space(24.0);
                    });
            });

        // Pindah halaman menutup semua modal yang terbuka.
        if let Some(page) = nav_to {
            if page != self.page {
                self.page = page;
                self.tx_form = None;
                self.cat_form = None;
                self.acc_form = None;
            }
        }

        transactions::show_form(self, &ctx);
        categories::show_form(self, &ctx);
        accounts::show_form(self, &ctx);
        self.show_confirm(&ctx);
        self.show_about(&ctx);

        self.purge_expired_toast();
        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
