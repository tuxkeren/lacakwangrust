#!/usr/bin/env bash
#
# Membangun lacakwangrust.exe (Windows 64-bit) dari Linux, tanpa perlu
# memasang Rust di Windows sama sekali. Hasilnya satu file .exe mandiri
# yang cukup di-double-click.
#
# Sekali saja, siapkan toolchain silang LLVM-MinGW (Linux-hosted):
#
#   curl -L -o /tmp/llvm-mingw.tar.xz \
#     https://github.com/mstorsjo/llvm-mingw/releases/download/20261006/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64.tar.xz
#   tar -xf /tmp/llvm-mingw.tar.xz -C /tmp
#   rustup target add x86_64-pc-windows-gnu
#
# Lalu jalankan skrip ini:
#
#   LLVM_MINGW_DIR=/tmp/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64 \
#     scripts/build-windows.sh
#
# Hasil: target/x86_64-pc-windows-gnu/release/lacakwangrust.exe
#
set -euo pipefail

cd "$(dirname "$0")/.."

: "${LLVM_MINGW_DIR:=/tmp/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64}"
if [ ! -d "$LLVM_MINGW_DIR/bin" ]; then
    echo "Toolchain LLVM-MinGW tidak ditemukan di: $LLVM_MINGW_DIR" >&2
    echo "Set variabel LLVM_MINGW_DIR ke folder hasil ekstrak llvm-mingw." >&2
    exit 1
fi
LLVM_MINGW_DIR="$(cd "$LLVM_MINGW_DIR" && pwd)"

export PATH="$LLVM_MINGW_DIR/bin:$PATH"

# rustc untuk target windows-gnu meminta -lgcc dan -lgcc_eh, sedangkan
# LLVM-MinGW memakai compiler-rt + libunwind. Kita buat "shim" berupa
# salinan dengan nama yang diharapkan rustc.
SHIM_DIR="${SHIM_DIR:-/tmp/lacakwangrust-win-shim}"
BUILTINS="$(ls "$LLVM_MINGW_DIR"/lib/clang/*/lib/windows/libclang_rt.builtins-x86_64.a 2>/dev/null | head -1)"
if [ -z "$BUILTINS" ]; then
    echo "libclang_rt.builtins-x86_64.a tidak ditemukan di $LLVM_MINGW_DIR" >&2
    exit 1
fi
mkdir -p "$SHIM_DIR"
cp -f "$BUILTINS" "$SHIM_DIR/libgcc.a"
cp -f "$LLVM_MINGW_DIR/x86_64-w64-mingw32/lib/libunwind.a" "$SHIM_DIR/libgcc_eh.a"

export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-clang
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-clang
export AR_x86_64_pc_windows_gnu=llvm-ar
export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-L$SHIM_DIR"

cargo build --release --target x86_64-pc-windows-gnu "$@"

echo
echo "Selesai: target/x86_64-pc-windows-gnu/release/lacakwangrust.exe"
