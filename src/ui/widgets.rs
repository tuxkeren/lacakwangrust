//! Kumpulan widget kustom yang meniru tampilan komponen TailwindCSS
//! pada versi PHP (kartu, badge, progress bar, tabel, tombol, modal).

use eframe::egui::{
    self, Align, Align2, Color32, CursorIcon, FontId, Layout, Rect, RichText, Sense, Shadow,
    Stroke, Ui,
};

use super::theme;

/// Padding horizontal di tepi kiri/kanan tabel (setara `px-6` = 24px).
pub const CELL_PAD: f32 = 24.0;

/// Jarak antar kolom di dalam tabel, supaya teks kolom tidak berdempetan.
const COL_GAP: f32 = 8.0;

const WHITE: Color32 = Color32::WHITE;

/// Kartu putih dengan sudut membulat dan bayangan halus (`rounded-lg shadow-sm`).
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let full = ui.available_width();
    egui::Frame::new()
        .fill(WHITE)
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::same(20))
        .stroke(Stroke::new(1.0, theme::GRAY_100))
        .shadow(Shadow {
            color: Color32::from_black_alpha(12),
            ..Default::default()
        })
        .show(ui, |ui| {
            ui.set_min_width(full - 40.0);
            add(ui)
        })
        .inner
}

/// Kartu putih dengan garis aksen tebal di sisi kiri (`border-l-4`).
pub fn card_accent<R>(ui: &mut Ui, accent: Color32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let full = ui.available_width();
    let resp = egui::Frame::new()
        .fill(WHITE)
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin {
            left: 24,
            right: 20,
            top: 20,
            bottom: 20,
        })
        .stroke(Stroke::new(1.0, theme::GRAY_100))
        .shadow(Shadow {
            color: Color32::from_black_alpha(12),
            ..Default::default()
        })
        .show(ui, |ui| {
            ui.set_min_width(full - 44.0);
            add(ui)
        });

    let rect = resp.response.rect;
    let bar = Rect::from_min_size(rect.min, egui::vec2(4.0, rect.height()));
    ui.painter().rect_filled(
        bar,
        egui::CornerRadius {
            nw: 8,
            sw: 8,
            ne: 0,
            se: 0,
        },
        accent,
    );

    resp.inner
}

/// Judul halaman (`text-2xl font-bold`).
pub fn page_title(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(24.0).strong().color(theme::GRAY_900));
}

/// Judul di dalam kartu (`text-lg font-semibold`).
pub fn section_title(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(18.0).strong().color(theme::GRAY_900));
}

/// Teks sekunder kecil (`text-sm text-gray-500`).
pub fn muted(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(13.0).color(theme::GRAY_500));
}

/// Label field form (`text-sm font-medium text-gray-700`).
pub fn field_label(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(13.0).strong().color(theme::GRAY_700));
}

/// Titik warna kecil (penanda warna kategori/akun).
pub fn dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 6.0, color);
}

/// Badge/pill kecil (`px-2 py-0.5 text-xs rounded-full`).
pub fn badge(ui: &mut Ui, text: &str, bg: Color32, fg: Color32) {
    let font = FontId::proportional(11.0);
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, fg);
    let size = egui::vec2(galley.size().x + 16.0, 20.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, egui::CornerRadius::same(10), bg);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, fg);
}

/// Progress bar dengan track abu-abu membulat (`w-full bg-gray-200 rounded-full h-2`).
pub fn progress_bar(ui: &mut Ui, fraction: f32, color: Color32) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 8.0), Sense::hover());
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(4), theme::GRAY_200);

    let f = fraction.clamp(0.0, 1.0);
    if f > 0.0 {
        let fill = Rect::from_min_size(rect.min, egui::vec2(rect.width() * f, rect.height()));
        ui.painter()
            .rect_filled(fill, egui::CornerRadius::same(4), color);
    }
}

/// Gaya tombol.
#[derive(Clone, Copy)]
pub struct ButtonStyle {
    pub fill: Color32,
    pub hover: Color32,
    pub text: Color32,
}

impl ButtonStyle {
    pub const fn primary() -> Self {
        Self {
            fill: theme::PRIMARY,
            hover: theme::BLUE_700,
            text: WHITE,
        }
    }

    pub const fn danger() -> Self {
        Self {
            fill: theme::RED_600,
            hover: theme::RED_700,
            text: WHITE,
        }
    }

    pub const fn secondary() -> Self {
        Self {
            fill: theme::GRAY_100,
            hover: theme::GRAY_200,
            text: theme::GRAY_700,
        }
    }
}

/// Tombol dengan warna solid & efek hover (meniru tombol Tailwind).
pub fn styled_button(ui: &mut Ui, text: &str, style: ButtonStyle, width: Option<f32>) -> egui::Response {
    let font = FontId::proportional(14.0);
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, style.text);
    let height = 38.0;
    let w = width
        .unwrap_or(galley.size().x + 32.0)
        .max(galley.size().x + 20.0);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, height), Sense::click());
    let fill = if resp.hovered() {
        style.hover
    } else {
        style.fill
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(8), fill);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, style.text);
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Tautan teks (meniru `<a>` berwarna).
pub fn link(ui: &mut Ui, text: &str, color: Color32, hover: Color32) -> egui::Response {
    let font = FontId::proportional(14.0);
    let measure = ui.painter().layout_no_wrap(text.to_owned(), font.clone(), color);
    let (rect, resp) = ui.allocate_exact_size(measure.size() + egui::vec2(0.0, 6.0), Sense::click());
    let now = if resp.hovered() { hover } else { color };
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, now);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, now);
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Item navigasi di navbar; halaman aktif diberi latar biru muda.
pub fn nav_button(ui: &mut Ui, text: &str, active: bool) -> egui::Response {
    let font = FontId::proportional(14.0);
    let color = if active { theme::PRIMARY } else { theme::GRAY_700 };
    let measure = ui.painter().layout_no_wrap(text.to_owned(), font.clone(), color);
    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(measure.size().x + 28.0, 36.0), Sense::click());

    if active {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(6), theme::BLUE_50);
    } else if resp.hovered() {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(6), theme::GRAY_100);
    }

    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, color);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Tombol pilihan besar untuk tipe transaksi/kategori.
pub fn choice_card(
    ui: &mut Ui,
    width: f32,
    text: &str,
    selected: bool,
    accent: Color32,
    accent_bg: Color32,
) -> egui::Response {
    let font = FontId::proportional(14.0);
    let color = if selected { accent } else { theme::GRAY_700 };
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, 46.0), Sense::click());

    let fill = if selected {
        accent_bg
    } else if resp.hovered() {
        theme::GRAY_50
    } else {
        WHITE
    };
    let stroke_color = if selected { accent } else { theme::GRAY_200 };
    let stroke_w = if selected { 2.0 } else { 2.0 };

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(8), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(8),
        Stroke::new(stroke_w, stroke_color),
        egui::StrokeKind::Inside,
    );

    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, color);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
    resp.on_hover_cursor(CursorIcon::PointingHand)
}

/// Perataan sel tabel.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellAlign {
    Left,
    Right,
}

/// Kartu putih tanpa padding dalam (dipakai untuk tabel yang menempel penuh).
pub fn card_flush<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let full = ui.available_width();
    egui::Frame::new()
        .fill(WHITE)
        .corner_radius(egui::CornerRadius::same(8))
        .stroke(Stroke::new(1.0, theme::GRAY_100))
        .shadow(Shadow {
            color: Color32::from_black_alpha(12),
            ..Default::default()
        })
        .show(ui, |ui| {
            ui.set_min_width(full - 2.0);
            add(ui)
        })
        .inner
}

/// Area tabel tanpa padding tepi, yaitu tempat kolom-kolom dihitung.
fn table_inner(rect: Rect) -> Rect {
    Rect::from_min_max(
        egui::pos2(rect.left() + CELL_PAD, rect.top()),
        egui::pos2(rect.right() - CELL_PAD, rect.bottom()),
    )
}

/// Menghitung rect setiap kolom di dalam area tabel.
///
/// Header dan isi baris memakai fungsi yang sama persis, sehingga kolomnya
/// selalu lurus (sebelumnya header memakai pecahan sedangkan isi baris
/// memakai alokasi berurutan, sehingga makin ke kanan makin bergeser).
fn column_rects(inner: Rect, columns: &[(&str, f32, CellAlign)]) -> Vec<Rect> {
    let count = columns.len();
    let mut rects = Vec::with_capacity(count);
    let mut x = inner.left();

    for (i, (_, frac, _)) in columns.iter().enumerate() {
        let width = inner.width() * frac;
        let left = if i == 0 { x } else { x + COL_GAP / 2.0 };
        let right = if i + 1 == count {
            x + width
        } else {
            x + width - COL_GAP / 2.0
        };
        rects.push(Rect::from_min_max(
            egui::pos2(left, inner.top()),
            egui::pos2(right, inner.bottom()),
        ));
        x += width;
    }

    rects
}

/// Baris header tabel.
pub fn table_header(ui: &mut Ui, columns: &[(&str, f32, CellAlign)]) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 40.0), Sense::hover());
    // Sudut atas dibuat membulat mengikuti kartu (`overflow-hidden` di versi PHP).
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius {
            nw: 8,
            ne: 8,
            sw: 0,
            se: 0,
        },
        theme::GRAY_50,
    );
    ui.painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, theme::GRAY_200));

    let rects = column_rects(table_inner(rect), columns);
    for ((label, _, align), cell) in columns.iter().zip(rects) {
        let (anchor, ax) = match align {
            CellAlign::Left => (Align2::LEFT_CENTER, cell.left()),
            CellAlign::Right => (Align2::RIGHT_CENTER, cell.right()),
        };
        ui.painter().text(
            egui::pos2(ax, cell.center().y),
            anchor,
            *label,
            FontId::proportional(11.0),
            theme::GRAY_500,
        );
    }
}

/// Baris tabel dengan latar hover dan garis pemisah bawah.
///
/// Closure menerima daftar rect kolom (urutannya sama dengan `columns`),
/// lalu tiap sel digambar dengan [`cell_at`].
pub fn table_row<R>(
    ui: &mut Ui,
    height: f32,
    columns: &[(&str, f32, CellAlign)],
    add: impl FnOnce(&mut Ui, &[Rect]) -> R,
) -> (R, egui::Response) {
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    if resp.hovered() {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(0), theme::GRAY_50);
    }
    ui.painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, theme::GRAY_200));

    let rects = column_rects(table_inner(rect), columns);
    let out = add(ui, &rects);
    (out, resp)
}

/// Menggambar satu sel pada rect kolom tertentu.
///
/// Memakai rect kolom secara langsung (bukan alokasi berurutan), jadi posisi
/// sel selalu sama dengan posisi header.
pub fn cell_at(ui: &mut Ui, rect: Rect, align: CellAlign, add: impl FnOnce(&mut Ui)) {
    let layout = match align {
        CellAlign::Left => Layout::left_to_right(Align::Center),
        CellAlign::Right => Layout::right_to_left(Align::Center),
    };
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(layout));
    child.spacing_mut().item_spacing.x = 8.0;
    add(&mut child);
}

/// Kotak input teks satu baris.
pub fn text_input(ui: &mut Ui, value: &mut String, hint: &str, width: f32) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(hint)
            .desired_width(width)
            .font(FontId::proportional(14.0))
            .margin(egui::Margin::symmetric(12, 8)),
    )
}

/// Kotak input teks banyak baris.
pub fn text_area(ui: &mut Ui, value: &mut String, hint: &str, rows: usize, width: f32) -> egui::Response {
    ui.add(
        egui::TextEdit::multiline(value)
            .hint_text(hint)
            .desired_width(width)
            .desired_rows(rows)
            .font(FontId::proportional(14.0))
            .margin(egui::Margin::symmetric(12, 8)),
    )
}

/// Pembungkus dropdown (ComboBox) agar konsisten dengan gaya lainnya.
pub fn combo(ui: &mut Ui, id: &str, selected: &str, width: f32, add: impl FnOnce(&mut Ui)) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(RichText::new(selected).size(14.0))
        .width(width)
        .show_ui(ui, add);
}

/// Pesan kosong terpusat dengan ikon dan tombol aksi opsional.
///
/// Mengembalikan `true` bila tombol aksi diklik.
pub fn empty_state(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    subtitle: &str,
    action: Option<&str>,
) -> bool {
    let mut clicked = false;
    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.label(RichText::new(icon).size(40.0).color(theme::GRAY_400));
        ui.add_space(4.0);
        ui.label(RichText::new(title).size(14.0).strong().color(theme::GRAY_900));
        ui.label(RichText::new(subtitle).size(14.0).color(theme::GRAY_500));
        if let Some(label) = action {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                let total = ui.available_width();
                let w = 220.0_f32.min(total);
                clicked = styled_button(ui, label, ButtonStyle::primary(), Some(w)).clicked();
            });
        }
        ui.add_space(24.0);
    });
    clicked
}

/// Menampilkan modal terpusat di atas lapisan gelap.
///
/// `size` menentukan lebar panel; tinggi mengikuti isi.
pub fn modal<R>(
    ctx: &egui::Context,
    id: &str,
    size: egui::Vec2,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let frame = egui::Frame::NONE
        .fill(Color32::WHITE)
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(24))
        .stroke(Stroke::new(1.0, theme::GRAY_100));

    egui::Modal::new(egui::Id::new(id))
        .frame(frame)
        .backdrop_color(Color32::from_black_alpha(110))
        .show(ctx, |ui| {
            ui.set_width(size.x);
            add(ui)
        })
        .inner
}

/// Kartu statistik dengan aksen kiri berwarna (meniru `border-l-4`).
#[allow(clippy::too_many_arguments)]
pub fn stat_card(
    ui: &mut Ui,
    title: &str,
    value: &str,
    value_color: Color32,
    subtitle: &str,
    accent: Color32,
    icon_bg: Color32,
    icon_fg: Color32,
    glyph: &str,
) {
    card_accent(ui, accent, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(title).size(13.0).strong().color(theme::GRAY_600));
                ui.add_space(2.0);
                ui.label(RichText::new(value).size(24.0).strong().color(value_color));
                ui.add_space(2.0);
                ui.label(RichText::new(subtitle).size(11.0).color(theme::GRAY_500));
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(52.0, 52.0), Sense::hover());
                ui.painter().circle_filled(rect.center(), 26.0, icon_bg);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    glyph,
                    FontId::proportional(22.0),
                    icon_fg,
                );
            });
        });
    });
}

/// Kotak pesan kesalahan di dalam konten.
pub fn error_box(ui: &mut Ui, message: &str) {
    egui::Frame::new()
        .fill(theme::RED_50)
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::symmetric(16, 12))
        .stroke(Stroke::new(1.0, theme::RED_500))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(RichText::new(message).size(14.0).color(theme::RED_800));
        });
}

/// Ikon panah kembali (untuk header form).
pub fn back_arrow(ui: &mut Ui) -> egui::Response {
    let font = FontId::proportional(20.0);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(28.0, 28.0), Sense::click());
    let color = if resp.hovered() {
        theme::GRAY_700
    } else {
        theme::GRAY_500
    };
    let galley = ui.painter().layout_no_wrap("\u{00AB}".to_owned(), font, color);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
    resp.on_hover_cursor(CursorIcon::PointingHand)
}
