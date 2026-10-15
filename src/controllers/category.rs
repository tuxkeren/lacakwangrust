//! Controller kategori — padanan dari `controllers/CategoryController.php`.

use rusqlite::Connection;

use crate::models::Category;

/// Data halaman Manajemen Kategori.
#[derive(Clone, Debug, Default)]
pub struct CategoryListData {
    pub categories: Vec<Category>,
    pub income_categories: Vec<Category>,
    pub expense_categories: Vec<Category>,
}

pub struct CategoryController;

impl CategoryController {
    pub fn index(conn: &Connection) -> rusqlite::Result<CategoryListData> {
        // Ambil per tipe langsung dari database, lalu gabungkan untuk daftar
        // lengkap — sekaligus menampilkan jumlah kategori di header.
        let income_categories = Category::by_type(conn, "income")?;
        let expense_categories = Category::by_type(conn, "expense")?;
        let categories: Vec<Category> = income_categories
            .iter()
            .chain(expense_categories.iter())
            .cloned()
            .collect();

        Ok(CategoryListData {
            categories,
            income_categories,
            expense_categories,
        })
    }

    /// Ambil satu kategori apa adanya (dipakai untuk mengisi form edit).
    pub fn detail(conn: &Connection, id: i64) -> rusqlite::Result<Option<Category>> {
        Category::get(conn, id)
    }

    /// Simpan kategori baru (id `None`) atau perbarui kategori yang ada.
    pub fn save(
        conn: &Connection,
        id: Option<i64>,
        name: &str,
        kind: &str,
        color: &str,
    ) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("Nama kategori wajib diisi".into());
        }
        if kind != "income" && kind != "expense" {
            return Err("Tipe kategori tidak valid".into());
        }
        let color = normalize_color(color);

        match id {
            Some(id) if id > 0 => {
                Category::update(conn, id, name, &color).map_err(|e| e.to_string())
            }
            _ => Category::create(conn, name, kind, &color)
                .map(|_| ())
                .map_err(|e| e.to_string()),
        }
    }

    /// Hapus kategori. Berbeda dengan versi PHP, di sini kita menolak
    /// penghapusan bila masih ada transaksi agar data tidak ikut terhapus.
    pub fn delete(conn: &Connection, id: i64) -> Result<(), String> {
        let used: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM transactions WHERE category_id = ?1",
                rusqlite::params![id],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())?;

        if used > 0 {
            return Err(format!(
                "Tidak bisa menghapus: masih ada {used} transaksi memakai kategori ini"
            ));
        }

        Category::delete(conn, id).map_err(|e| e.to_string())
    }
}

/// Memastikan warna berformat `#RRGGBB`, jika tidak valid pakai warna primary.
fn normalize_color(color: &str) -> String {
    let c = color.trim();
    let hex = c.trim_start_matches('#');
    if hex.len() == 6 && hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        format!("#{}", hex.to_uppercase())
    } else {
        "#3B82F6".to_string()
    }
}
