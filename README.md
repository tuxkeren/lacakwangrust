# LacakWangRust - Aplikasi Pencatatan Keuangan Sederhana (Native Desktop)

Versi **native (desktop)** dari aplikasi PHP OOP **PelacakDuit**, ditulis ulang
dengan **Rust** + **egui/eframe**. Tampilan, alur kerja, dan struktur kode
sengaja dibuat mengikuti versi PHP-nya, tetapi berjalan sebagai aplikasi
desktop biasa — **tanpa browser, tanpa webview, tanpa JavaScript**.

- **Bahasa**: Rust (edition 2021)
- **GUI**: [egui](https://github.com/emilk/egui) / eframe 0.36 dengan backend OpenGL (`glow`)
- **Database**: SQLite (via `rusqlite`, library SQLite di-*bundle*)
- **Arsitektur**: tetap MVC seperti versi PHP — `models` (data), `controllers` (logika), `ui` (tampilan)

## ⚠️ Disclaimer

Sama seperti versi PHP-nya, aplikasi ini dibuat untuk **tujuan belajar** dan
**tidak memiliki fitur autentikasi/login**.

- ❌ Tidak ada sistem login / manajemen pengguna
- ❌ Database tidak dienkripsi (SQLite biasa di folder `database/`)
- ✅ Cocok untuk belajar konsep MVC, OOP, CRUD, dan GUI native
- ✅ Untuk pemakaian pribadi di komputer sendiri

**Jangan gunakan untuk data sensitif atau dibagikan ke jaringan tanpa
menambahkan lapisan pengamanan terlebih dahulu.**

## 🚀 Cara Menjalankan

### Prasyarat

- Rust toolchain (disarankan lewat [rustup](https://rustup.rs))
- Linux: paket pengembangan untuk Wayland/X11 (umumnya sudah tersedia di
  desktop modern)

### Mode development

```bash
cd lacakwangrust
cargo run
```

### Build rilis (binary mandiri)

```bash
cargo build --release
./target/release/lacakwangrust
```

Binary hasil build sudah mencakup SQLite di dalamnya, jadi tidak perlu
memasang SQLite secara terpisah.

## 🪟 Versi Windows (siap pakai, tanpa install Rust)

Windows **tidak perlu memasang Rust** — unduh paket siap pakai dari halaman
**[Releases](https://github.com/tuxkeren/lacakwangrust/releases)**:

| Berkas | Isi |
| --- | --- |
| `lacakwangrust-windows-x86_64.zip` | aplikasi + panduan singkat (`BACA-SAYA.txt`) |
| `lacakwangrust.exe` | aplikasinya saja, kalau ingin unduh satu berkas |

Ekstrak, lalu klik dua kali `lacakwangrust.exe`. Syarat: **Windows 10/11
64-bit**. Saat pertama dibuka, Windows SmartScreen mungkin memperingatkan
karena file belum ditandatangani digital — pilih **More info → Run anyway**.

Ikon aplikasi sudah menyatu di dalam `.exe` (muncul di File Explorer,
taskbar, shortcut, dan judul jendela) — tidak ada file ikon terpisah
yang perlu ikut disalin.

Folder `database/` **tidak disertakan** dalam paket; aplikasi membuatnya
sendiri (kosong, lalu mengisi tabel + data awal) saat pertama dijalankan.


### Membangun ulang `.exe` dari Linux (opsional)

Menggunakan *cross-compile* dengan LLVM-MinGW:

```bash
# sekali saja: siapkan toolchain silang
curl -L -o /tmp/llvm-mingw.tar.xz \
  https://github.com/mstorsjo/llvm-mingw/releases/download/20261006/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64.tar.xz
tar -xf /tmp/llvm-mingw.tar.xz -C /tmp
rustup target add x86_64-pc-windows-gnu

# build .exe (hasil: target/x86_64-pc-windows-gnu/release/lacakwangrust.exe)
LLVM_MINGW_DIR=/tmp/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64 scripts/build-windows.sh
```

Catatan teknis:

- `#![windows_subsystem = "windows"]` pada build rilis membuat aplikasi
  berjalan sebagai aplikasi GUI murni, sehingga **tidak ada jendela konsol
  hitam** saat di-double-click.
- Kalau database gagal dibuka, Windows menampilkan **kotak dialog**
  (bukan pesan ke konsol yang tidak terlihat).
- Ikon aplikasi disisipkan ke dalam `.exe` oleh `build.rs` (memakai
  `winresource` + `assets/icon.ico`), plus metadata versi/produk yang
  terlihat di *Properties → Details*. Ikon yang sama juga dipasang ke
  jendela aplikasi saat berjalan, jadi tampil benar di title bar & taskbar.
- Aplikasi hanya bergantung pada DLL bawaan Windows (Universal C Runtime
  `api-ms-win-crt-*`, `opengl32`, `user32`, dst.), jadi **tidak ada DLL
  tambahan** yang perlu ikut disalin.

## 🗄️ Konfigurasi Database

Secara default aplikasi memakai file SQLite:

```
database/lacakwangrust.db
```

Folder `database/` dan file-nya **dibuat otomatis** saat aplikasi pertama kali
dijalankan (tabel + data awal ikut dibuat).

Lokasi database bisa diubah lewat environment variable `LACAKWANGRUST_DB`.
Ini memungkinkan aplikasi membaca database milik versi PHP:

```bash
# memakai database versi PHP
LACAKWANGRUST_DB=../pelacakduit/database/pelacakduit.db cargo run

# atau menyimpan di lokasi lain
LACAKWANGDUIT_DB=~/keuangan.db cargo run   # (contoh, sesuaikan nama variabel)
```

> Catatan: variabel yang benar adalah `LACAKWANGRUST_DB`.

## 📁 Struktur Project

```
lacakwangrust/
├── Cargo.toml                  # Dependensi & profil build
├── build.rs                    # Menyisipkan ikon & metadata ke .exe Windows
├── LICENSE                     # MIT
├── assets/                     # Ikon aplikasi (dibuat oleh tools/make_icon.py)
│   ├── icon.ico                #   ikon multi-ukuran untuk .exe Windows
│   ├── icon-64.rgba            #   ikon jendela 64x64 (RGBA mentah)
│   └── icon.png / icon-512.png #   pratinjau
├── database/
│   └── lacakwangrust.db        # SQLite (dibuat otomatis, tidak di-commit)
├── dist/                       # Hasil build lokal untuk dibagikan (tidak di-commit)
│   ├── lacakwangrust-windows-x86_64/   # .exe + panduan (siap salin)
│   └── lacakwangrust-windows-x86_64.zip
├── scripts/
│   └── build-windows.sh        # Cross-compile .exe dari Linux
├── tools/
│   └── make_icon.py            # Pembuat ikon (.ico/.png/.rgba)
└── src/
    ├── main.rs                 # Entry point (window + inisialisasi DB)
    ├── config.rs               # Konfigurasi aplikasi + lokasi database
    ├── util.rs                 # Format rupiah/tanggal, konversi warna, parse angka
    ├── db/
    │   ├── mod.rs              # Pembungkus koneksi (setara Database.php)
    │   └── schema.rs           # Auto-create tabel + data awal (setara Schema.php)
    ├── models/                 # Padanan folder `models/` PHP
    │   ├── account.rs          # Account + ringkasan per akun
    │   ├── category.rs         # Category (CRUD)
    │   └── transaction.rs      # Transaction (CRUD + statistik)
    ├── controllers/            # Padanan folder `controllers/` PHP
    │   ├── dashboard.rs
    │   ├── transaction.rs
    │   ├── category.rs
    │   └── account.rs
    └── ui/                     # Padanan folder `views/` PHP (dirender dengan egui)
        ├── app.rs              # Kerangka aplikasi: navigasi, modal, notifikasi
        ├── theme.rs            # Palet warna Tailwind + gaya egui
        ├── widgets.rs          # Widget mirip Tailwind (kartu, badge, tombol, tabel)
        ├── dashboard.rs
        ├── transactions.rs
        ├── categories.rs
        └── accounts.rs
```

## ✨ Fitur

### Dashboard
- Ringkasan 30 hari terakhir: **Total Pemasukan**, **Total Pengeluaran**, **Saldo**
- Breakdown per kategori (pemasukan & pengeluaran) dengan progress bar
- Daftar 10 transaksi terbaru

### Transaksi
- CRUD lengkap: Tambah, Edit, Hapus (dengan dialog konfirmasi native)
- Filter: Semua / Pemasukan / Pengeluaran
- Paginasi 10 baris per halaman
- Validasi form (jumlah, kategori, tanggal, akun)
- Kolom: Tanggal, Kategori, Akun, Deskripsi, Tipe, Jumlah, Aksi

### Kategori
- CRUD lengkap lewat modal
- Warna kustom per kategori (palet + input kode hex)
- Tipe: Pemasukan / Pengeluaran
- Menolak penghapusan bila kategori masih dipakai transaksi

### Akun / Dompet
- CRUD lengkap lewat modal
- Tipe: Tunai, Bank, E-Wallet, Lainnya
- Saldo awal + saldo berjalan (saldo awal + masuk − keluar) per akun
- Ringkasan total pemasukan/pengeluaran/saldo gabungan semua akun

## 🎨 Tampilan

- Palet warna memakai nilai heksadesimal **TailwindCSS yang sama** dengan versi PHP
  (mis. primary `#3B82F6`, hijau `#10B981`, merah `#EF4444`, latar `#F9FAFB`).
- Widget ditulis khusus agar menyerupai komponen Tailwind: kartu bershadow,
  badge bulat, progress bar, tabel bergaris, tombol solid, dan modal ber-overlay.
- Semua teks antarmuka berbahasa Indonesia, format uang `Rp 1.234.567`,
  format tanggal `dd/mm/yyyy`.

## 🗄️ Skema Database

Identik dengan versi PHP (`config/Schema.php`).

```sql
categories (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       VARCHAR(100) NOT NULL,
    type       VARCHAR(10)  NOT NULL CHECK (type IN ('income','expense')),
    color      VARCHAR(7)   DEFAULT '#3B82F6',
    created_at DATETIME     DEFAULT CURRENT_TIMESTAMP
)

accounts (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       VARCHAR(100) NOT NULL,
    type       VARCHAR(50)  DEFAULT 'bank',
    balance    DECIMAL(15,2) DEFAULT 0,
    created_at DATETIME     DEFAULT CURRENT_TIMESTAMP
)

transactions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER      NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    account_id  INTEGER      DEFAULT 1 REFERENCES accounts(id) ON DELETE SET NULL,
    amount      DECIMAL(15,2) NOT NULL,
    type        VARCHAR(10)  NOT NULL CHECK (type IN ('income','expense')),
    description TEXT,
    date        DATE         NOT NULL,
    created_at  DATETIME     DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME     DEFAULT CURRENT_TIMESTAMP
)
```

Data awal (hanya dibuat saat tabel masih kosong), sama seperti versi PHP:

- **Akun**: Tunai / Cash, Bank Mandiri, BSI (Bank Syariah Indonesia),
  E-Wallet (GoPay/OVO)
- **Kategori**: Gaji, Penjualan, Bonus (pemasukan); Makanan, Transportasi,
  Belanja, Tagihan, Lainnya (pengeluaran)

## 🔧 Konfigurasi

Bagian `Config` di `src/config.rs` meniru `config/config.php`:

| Field             | Default        | Keterangan                        |
| ----------------- | -------------- | --------------------------------- |
| `app_name`        | `LacakWangRust`| Nama aplikasi (judul & navbar)    |
| `version`         | `1.0.0`        | Versi aplikasi (footer)           |
| `currency_symbol` | `Rp`           | Simbol mata uang                  |
| `currency_code`   | `IDR`          | Kode mata uang (footer)           |
| `per_page`        | `10`           | Baris per halaman daftar transaksi|

### Menunjuk ke database versi PHP

Karena skemanya sama, aplikasi ini bisa langsung membaca database
`pelacakduit.db` milik versi PHP:

```bash
LACAKWANGRUST_DB=/path/ke/pelacakduit/database/pelacakduit.db cargo run
```

> Catatan: bila database PHP masih memakai format lama (mis. belum punya
> kolom `account_id`), aplikasi akan menambahkannya otomatis saat pertama
> dibuka.

## 🆚 Perbedaan dengan Versi PHP

Beberapa hal sengaja berbeda karena keterbatasan/kebutuhan platform desktop:

| Hal                        | PHP (web)                        | Rust (native)                          |
| -------------------------- | -------------------------------- | -------------------------------------- |
| Penghapusan kategori       | Transaksi ikut terhapus (CASCADE)| **Ditolak** bila masih dipakai transaksi |
| Konfirmasi hapus           | `confirm()` bawaan browser       | Modal konfirmasi native                |
| Form tambah/edit           | Halaman / AJAX terpisah          | Modal di dalam aplikasi                |

## 📄 Lisensi

MIT License - Bebas digunakan & dimodifikasi.
