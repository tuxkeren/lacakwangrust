#!/usr/bin/env bash
#
# Mendaftarkan Lacakuang ke menu aplikasi Ubuntu/GNOME (desktop entry)
# beserta ikonnya di tema hicolor — semuanya tanpa sudo, cukup untuk
# satu pengguna (`~/.local/share`).
#
# Pemakaian:
#   scripts/install-linux-menu.sh              # pasang
#   scripts/install-linux-menu.sh --uninstall  # lepas
#
# Lingkungan (opsional):
#   LACAKWANGRUST_BIN   path binary yang dibuka menu
#                       (default: <proyek>/target/release/lacakwangrust)
#   LACAKWANGRUST_DIR   folder kerja aplikasi, tempat data
#                       `database/`, `settings/`, dan `laporan/` berada
#                       (default: folder tempat binary berada)
#
# Catatan: jalankan `cargo build --release` dulu bila binary belum ada.
set -euo pipefail

PROJEK="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${LACAKWANGRUST_BIN:-$PROJEK/target/release/lacakwangrust}"

APP_ID="lacakuang"
LOKASI_APLIKASI="$HOME/.local/share/applications"
IKON_DIR="$HOME/.local/share/icons/hicolor"
DESKTOP_FILE="$LOKASI_APLIKASI/$APP_ID.desktop"
LAUNCHER="$HOME/.local/share/$APP_ID/launch.sh"
UKURAN_IKON=(512 256 128 64 48 32)

perbarui_cache() {
    update-desktop-database "$LOKASI_APLIKASI" 2>/dev/null || true
    # index.theme diperlukan agar cache ikon hicolor bisa dibuat.
    if [ ! -f "$IKON_DIR/index.theme" ] && [ -f /usr/share/icons/hicolor/index.theme ]; then
        mkdir -p "$IKON_DIR"
        cp /usr/share/icons/hicolor/index.theme "$IKON_DIR/index.theme"
    fi
    gtk-update-icon-cache -f -t "$IKON_DIR" 2>/dev/null || true
}

hapus() {
    rm -f "$DESKTOP_FILE" "$LAUNCHER"
    rmdir "$HOME/.local/share/$APP_ID" 2>/dev/null || true
    for s in "${UKURAN_IKON[@]}"; do
        rm -f "$IKON_DIR/${s}x${s}/apps/$APP_ID.png"
    done
    perbarui_cache
    echo "Menu Lacakuang dilepas dari $LOKASI_APLIKASI."
}

if [ "${1:-}" = "--uninstall" ]; then
    hapus
    exit 0
fi

# ---- 1. Binary harus sudah ada ----
if [ ! -x "$BIN" ]; then
    echo "Binary tidak ditemukan: $BIN" >&2
    echo "Jalankan dulu: cargo build --release" >&2
    exit 1
fi
DIR_APLIKASI="$(cd "$(dirname "$BIN")" && pwd)"
BIN="$DIR_APLIKASI/$(basename "$BIN")"

# Peringatan bila source lebih baru daripada binary (menu akan membuka
# versi lama — persis masalah yang sering membingungkan).
TERBARU="$(find "$PROJEK/src" "$PROJEK/Cargo.toml" -type f -newer "$BIN" 2>/dev/null | head -1 || true)"
if [ -n "$TERBARU" ]; then
    echo "⚠ Binary lebih lama dari $TERBARU" >&2
    echo "  Jalankan 'cargo build --release' supaya menu membuka versi terbaru." >&2
fi

# ---- 2. Ikon ke tema hicolor ----
SUMBER_512="$PROJEK/assets/icon-512.png"
SUMBER_256="$PROJEK/assets/icon.png"
if [ ! -f "$SUMBER_512" ]; then
    echo "Ikon sumber tidak ada: $SUMBER_512" >&2
    exit 1
fi

for s in "${UKURAN_IKON[@]}"; do
    tujuan="$IKON_DIR/${s}x${s}/apps"
    mkdir -p "$tujuan"
    case "$s" in
    512) cp "$SUMBER_512" "$tujuan/$APP_ID.png" ;;
    256)
        if [ -f "$SUMBER_256" ]; then cp "$SUMBER_256" "$tujuan/$APP_ID.png"; else cp "$SUMBER_512" "$tujuan/$APP_ID.png"; fi
        ;;
    *)
        if command -v magick >/dev/null 2>&1; then
            magick "$SUMBER_512" -resize "${s}x${s}" "$tujuan/$APP_ID.png"
        elif command -v convert >/dev/null 2>&1; then
            convert "$SUMBER_512" -resize "${s}x${s}" "$tujuan/$APP_ID.png"
        else
            cp "$SUMBER_512" "$tujuan/$APP_ID.png"
        fi
        ;;
    esac
done

# ---- 3. Launcher kecil: menentukan folder kerja aplikasi ----
# Aplikasi menyimpan `database/`, `settings/`, dan `laporan/` relatif
# terhadap folder kerja, jadi titik start harus diatur di sini
# (desktop entry tidak punya kunci WorkingDirectory).
mkdir -p "$(dirname "$LAUNCHER")"
cat >"$LAUNCHER" <<EOF
#!/bin/sh
# Dibuat oleh scripts/install-linux-menu.sh — jangan diedit manual.
cd "$DIR_APLIKASI" || exit 1
exec "$BIN" "\$@"
EOF
chmod +x "$LAUNCHER"

# ---- 4. Desktop entry ----
mkdir -p "$LOKASI_APLIKASI"
cat >"$DESKTOP_FILE" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Lacakuang
GenericName=Pencatatan Keuangan
Comment=Pencatatan keuangan sederhana: pemasukan, pengeluaran, dan laporan PDF
Exec=$LAUNCHER
Icon=$APP_ID
Terminal=false
Categories=Office;Finance;
Keywords=uang;keuangan;laporan;pemasukan;pengeluaran;dompet;
StartupNotify=true
StartupWMClass=lacakwangrust
EOF
chmod +x "$DESKTOP_FILE"

if command -v desktop-file-validate >/dev/null 2>&1; then
    desktop-file-validate "$DESKTOP_FILE"
fi

perbarui_cache

echo "✓ Menu dipasang : $DESKTOP_FILE"
echo "✓ Ikon          : $IKON_DIR/{512,256,128,64,48,32}x…/apps/$APP_ID.png"
echo "✓ Launcher      : $LAUNCHER"
echo "  Membuka       : $BIN"
echo "  Folder data   : $DIR_APLIKASI"
echo "Cari \"Lacakuang\" di menu aplikasi (bisa perlu logout/login bila belum muncul)."
