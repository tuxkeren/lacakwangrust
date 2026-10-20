# LacakWangRust - Aplikasi Pencatatan Keuangan Sederhana (Native Desktop)

Versi **native (desktop)** dari aplikasi PHP OOP **PelacakDuit**, ditulis ulang
dengan **Rust** + **egui/eframe**. Tampilan, alur kerja, dan struktur kode
sengaja dibuat mengikuti versi PHP-nya, tetapi berjalan sebagai aplikasi
desktop biasa — **tanpa browser, tanpa webview, tanpa JavaScript**.

- **Bahasa**: Rust (edition 2021)
- **GUI**: [egui](https://github.com/emilk/egui) / eframe 0.36 dengan backend OpenGL (`glow`)
- **Database**: SQLite (via `rusqlite`, library SQLite di-*bundle*)
- **Dialog file**: [rfd](https://github.com/PolyMeilex/rfd) — "Simpan Sebagai" memakai dialog bawaan sistem
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

### 🐧 Menu aplikasi (Ubuntu/GNOME)

Supaya Lacakuang muncul di menu aplikasi beserta ikonnya:

```bash
cargo build --release
scripts/install-linux-menu.sh
```

Pemasangan tanpa sudo, hanya untuk pengguna yang login:

- `~/.local/share/applications/lacakuang.desktop` — entri menu
- ikon PNG 32–512 px di tema `hicolor` (`lacakuang.png`)
- launcher kecil yang membuka binary release dengan folder kerja tempat
  data (`database/`, `settings/`, `laporan/`) berada

Lepas kembali dengan `scripts/install-linux-menu.sh --uninstall`.
Binary/lokasi bisa diganti lewat environment `LACAKWANGRUST_BIN` dan
`LACAKWANGRUST_DIR`. Setelah source diubah, jalankan ulang
`cargo build --release` — skrip akan memperingatkan bila binary lebih
lama dari source.

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
  terlihat di *Properties → Details* — termasuk **CompanyName** dan
  **LegalCopyright** (nama pembuat). Ikon yang sama juga dipasang ke
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
├── laporan/                    # Laporan PDF hasil cetak (dibuat otomatis, tidak di-commit)
├── settings/
│   └── tema.txt                # Preferensi tema terang/gelap (dibuat otomatis)
├── dist/                       # Hasil build lokal untuk dibagikan (tidak di-commit)
│   ├── lacakwangrust-windows-x86_64/   # .exe + panduan (siap salin)
│   └── lacakwangrust-windows-x86_64.zip
├── scripts/
│   ├── build-windows.sh        # Cross-compile .exe dari Linux
│   └── install-linux-menu.sh   # Pasang entri menu + ikon di Ubuntu/GNOME
├── tools/
│   └── make_icon.py            # Pembuat ikon (.ico/.png/.rgba)
└── src/
    ├── main.rs                 # Entry point (window + inisialisasi DB)
    ├── config.rs               # Konfigurasi aplikasi + lokasi database/tema/laporan
    ├── util.rs                 # Format rupiah/tanggal, konversi warna, parse angka
    ├── pdf.rs                  # Penulis laporan PDF dua kolom (tanpa mesin render)
    ├── db/
    │   ├── mod.rs              # Pembungkus koneksi (setara Database.php)
    │   └── schema.rs           # Auto-create tabel + data awal (setara Schema.php)
    ├── models/                 # Padanan folder `models/` PHP
    │   ├── account.rs          # Account + ringkasan per akun
    │   ├── category.rs         # Category (CRUD)
    │   ├── report.rs           # Periode & data laporan keuangan
    │   └── transaction.rs      # Transaction (CRUD + statistik)
    ├── controllers/            # Padanan folder `controllers/` PHP
    │   ├── dashboard.rs
    │   ├── transaction.rs
    │   ├── category.rs
    │   ├── account.rs
    │   └── report.rs           # Penyusun data laporan per periode
    └── ui/                     # Padanan folder `views/` PHP (dirender dengan egui)
        ├── app.rs              # Kerangka aplikasi: navigasi, modal, notifikasi
        ├── theme.rs            # Palet warna Tailwind (terang & gelap) + gaya egui
        ├── widgets.rs          # Widget mirip Tailwind (kartu, badge, tombol, tabel)
        ├── dashboard.rs
        ├── transactions.rs
        ├── categories.rs
        ├── accounts.rs
        └── report.rs           # Dialog pemilihan periode & pembuatan laporan PDF
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

### Laporan PDF
- Tombol **📄 Laporan PDF** di navbar membuka dialog pemilihan periode:
  **Semua Data**, **Bulan Ini**, atau **Tahun Ini**.
- Laporan dicetak dua kolom berdampingan: **Pemasukan** (kiri) dan
  **Pengeluaran** (kanan). Tiap baris memuat tanggal, deskripsi, dan jumlah.
- Di bagian bawah ditampilkan ringkasan (total pemasukan, total pengeluaran,
  selisih, saldo awal periode) dan **sisa saldo** pada akhir periode.
- Tombol **💾 Simpan Sebagai…** membuka dialog "Save As" bawaan sistem
  (xdg-portal di Linux, dialog file bawaan Windows di Windows); folder
  `laporan/` dengan nama `laporan-YYYYMMDD-HHMMSS.pdf` jadi titik awalnya,
  dan ekstensi `.pdf` ditambahkan otomatis bila pengguna tidak mengetiknya.
  Bila dialog dibatalkan, dialog laporan tetap terbuka tanpa pesan galat.
- Bila tidak ada transaksi pada periode terpilih, laporan tidak dibuat dan
  dialog menampilkan pesan galat.
- PDF ditulis langsung oleh aplikasi (font bawaan Helvetica), tanpa
  dependensi mesin render.

### Tema Terang / Gelap
- Tombol **🌙 / ☀️** di navbar mengganti tema terang ↔ gelap; warna teks
  serta seluruh komponen ikut menyesuaikan.
- Pilihan tema **diingat** di `settings/tema.txt` dan dipakai lagi saat
  aplikasi dibuka berikutnya.

## 🎨 Tampilan

- Palet warna memakai nilai heksadesimal **TailwindCSS yang sama** dengan versi PHP
  (mis. primary `#3B82F6`, hijau `#10B981`, merah `#EF4444`, latar `#F9FAFB`).
- Widget ditulis khusus agar menyerupai komponen Tailwind: kartu bershadow,
  badge bulat, progress bar, tabel bergaris, tombol solid, dan modal ber-overlay.
- Mendukung **mode terang & gelap**: semua warna diambil dari palet semantik
  (`text()`, `surface()`, `border()`, dst.) yang otomatis menyesuaikan tema aktif.
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

### Lokasi tema & laporan

Preferensi tema dan berkas laporan disimpan terpisah dari database (skema
database tetap identik dengan versi PHP), dan lokasinya bisa diubah lewat
environment variable:

| Variabel                | Default          | Keterangan                              |
| ----------------------- | ---------------- | --------------------------------------- |
| `LACAKWANGRUST_DB`      | `database/lacakwangrust.db` | Lokasi file database SQLite   |
| `LACAKWANGRUST_THEME`   | `settings/tema.txt`         | Lokasi berkas preferensi tema |
| `LACAKWANGRUST_LAPORAN` | `laporan`                   | Folder awal dialog laporan PDF |

## 🆚 Perbedaan dengan Versi PHP

Beberapa hal sengaja berbeda karena keterbatasan/kebutuhan platform desktop:

| Hal                        | PHP (web)                        | Rust (native)                          |
| -------------------------- | -------------------------------- | -------------------------------------- |
| Penghapusan kategori       | Transaksi ikut terhapus (CASCADE)| **Ditolak** bila masih dipakai transaksi |
| Konfirmasi hapus           | `confirm()` bawaan browser       | Modal konfirmasi native                |
| Form tambah/edit           | Halaman / AJAX terpisah          | Modal di dalam aplikasi                |

## 👤 Pembuat

**Athailah** ([@tuxkeren](https://github.com/tuxkeren)) — pengembang aplikasi ini.

Kredit pembuat juga tampil di dalam aplikasi:

- **Footer** setiap halaman: `Dibuat oleh Athailah (tuxkeren) · MIT License`
- Menu **Tentang** di kanan atas: info aplikasi, versi, teknologi, pembuat,
  tautan repositori, lisensi, dan lokasi berkas database.
- *Properties → Details* pada `lacakwangrust.exe` (CompanyName, LegalCopyright,
  Comments).

Ingin mengganti nama/kreditnya? Ubah satu tempat saja, di
`src/config.rs` (`Config::default()`), pada bagian `author`, `author_handle`,
`repo_url`, dan `license`.

## 📄 Lisensi

MIT License — bebas digunakan & dimodifikasi.

Copyright (c) 2026 Athailah.
