#!/usr/bin/env python3
"""Membuat ikon aplikasi LacakWangRust (ikon .ico untuk Windows + PNG).

Desain: "app tile" biru (gradasi Tailwind blue-500 -> blue-700) berisi dompet
putih dengan pengait berkancing di sisi kanan, dan koin amber di depan pada
sudut kiri bawah.

Setiap ukuran dirender ulang dari bentuk vektor (bukan sekadar diperkecil),
dengan supersampling 4x supaya tepinya halus di segala ukuran.
"""
import struct

from PIL import Image, ImageDraw, ImageFilter

BASE = 1024.0  # kanvas desain

STOPS = [  # gradasi vertikal ubin
    (0.00, (59, 130, 246)),   # blue-500  #3B82F6
    (0.55, (37, 99, 235)),    # blue-600  #2563EB
    (1.00, (29, 78, 216)),    # blue-700  #1D4ED8
]

TILE_R = 232.0                    # radius sudut ubin (~23%, gaya app tile)
WALLET = (160.0, 240.0, 860.0, 710.0)   # badan dompet
WALLET_R = 90.0
CLASP = (640.0, 395.0, 900.0, 555.0)    # pengait di sisi kanan
CLASP_R = 60.0
BUTTON = (770.0, 475.0, 40.0)           # kancing: cx, cy, r
COIN = (250.0, 660.0, 130.0, 100.0, 20.0)  # cx, cy, r, r_cincin, tebal

WHITE = (255, 255, 255, 255)
CLASP_FILL = (191, 219, 254, 255)   # blue-200
BUTTON_FILL = (37, 99, 235, 255)    # blue-600
COIN_FACE = (251, 191, 36, 255)     # amber-400
COIN_RING = (217, 119, 6, 255)      # amber-600
SHADOW_RGBA = (15, 23, 42, 95)      # slate-900 transparan


def gradient(w: int, h: int) -> Image.Image:
    """Gradasi vertikal mulus (beberapa titik warna)."""
    strip = Image.new("RGB", (1, 256))
    px = strip.load()
    for i in range(256):
        t = i / 255.0
        for j in range(len(STOPS) - 1):
            t0, c0 = STOPS[j]
            t1, c1 = STOPS[j + 1]
            if t <= t1 or j == len(STOPS) - 2:
                u = 0.0 if t1 == t0 else (t - t0) / (t1 - t0)
                u = min(max(u, 0.0), 1.0)
                px[0, i] = tuple(round(c0[c] + (c1[c] - c0[c]) * u) for c in range(3))
                break
    return strip.resize((w, h), Image.Resampling.BILINEAR).convert("RGBA")


def render(size: int, ss: int = 4) -> Image.Image:
    """Menggambar ikon persegi berukuran `size` (RGBA)."""
    work = int(size * ss)
    k = size * ss / BASE  # faktor skala dari kanvas desain

    def rb(draw, box, radius, fill):
        draw.rounded_rectangle([v * k for v in box], radius=radius * k, fill=fill)

    def circ(draw, cx, cy, r, **kw):
        draw.ellipse([(cx - r) * k, (cy - r) * k, (cx + r) * k, (cy + r) * k], **kw)

    # ---- bayangan dompet (agar terlihat "mengambang" di atas ubin) ----
    shadow = Image.new("RGBA", (work, work), (0, 0, 0, 0))
    sd = ImageDraw.Draw(shadow)
    rb(sd, (WALLET[0], WALLET[1] + 14, WALLET[2], WALLET[3] + 14), WALLET_R, SHADOW_RGBA)
    rb(sd, (CLASP[0], CLASP[1] + 14, CLASP[2], CLASP[3] + 14), CLASP_R, SHADOW_RGBA)
    shadow = shadow.filter(ImageFilter.GaussianBlur(max(0.6, 20 * k)))

    canvas = Image.alpha_composite(gradient(work, work), shadow)
    d = ImageDraw.Draw(canvas)

    # ---- 1. badan dompet ----
    rb(d, WALLET, WALLET_R, WHITE)

    # ---- 2. pengait di sisi kanan (sedikit menonjol keluar) ----
    rb(d, CLASP, CLASP_R, CLASP_FILL)

    # ---- 3. kancing pengait ----
    bx, by, br = BUTTON
    circ(d, bx, by, br, fill=BUTTON_FILL)

    # ---- 4. koin di depan (sudut kiri bawah) + bayangannya ----
    cx, cy, cr, cring, cwidth = COIN
    coin_shadow = Image.new("RGBA", (work, work), (0, 0, 0, 0))
    circ(
        ImageDraw.Draw(coin_shadow),
        cx,
        cy + 8,
        cr,
        fill=(15, 23, 42, 90),
    )
    coin_shadow = coin_shadow.filter(ImageFilter.GaussianBlur(max(0.6, 16 * k)))
    canvas = Image.alpha_composite(canvas, coin_shadow)
    d = ImageDraw.Draw(canvas)
    circ(d, cx, cy, cr, fill=COIN_FACE)
    circ(d, cx, cy, cring, outline=COIN_RING, width=max(1, round(cwidth * k)))

    # ---- sudut membulat: perkecil RGB dulu (tanpa alpha, supaya tidak ada
    #      pinggiran gelap), lalu tempelkan alpha halus ----
    out = canvas.convert("RGB").resize((size, size), Image.Resampling.LANCZOS)
    out = out.convert("RGBA")
    mask = Image.new("L", (work, work), 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        [0, 0, work - 1, work - 1], radius=TILE_R * k, fill=255
    )
    out.putalpha(mask.resize((size, size), Image.Resampling.LANCZOS))
    return out


def bmp_for_ico(im: Image.Image) -> bytes:
    """Citra format ICO: BITMAPINFOHEADER + piksel BGRA + AND mask."""
    w, h = im.size
    px = im.load()
    xor = bytearray()
    for y in range(h - 1, -1, -1):
        for x in range(w):
            r, g, b, a = px[x, y]
            xor += bytes((b, g, r, a))
    stride = ((w + 31) // 32) * 4
    return (
        struct.pack("<IiiHHIIiiII", 40, w, h * 2, 1, 32, 0, len(xor), 0, 0, 0, 0)
        + bytes(xor)
        + bytes(stride * h)
    )


def write_ico(path: str, sizes=(16, 20, 24, 32, 40, 48, 64, 128, 256)) -> None:
    blobs = [(s, bmp_for_ico(render(s))) for s in sizes]
    n = len(blobs)
    out = struct.pack("<HHH", 0, 1, n)
    offset = 6 + 16 * n
    dir_entries = b""
    data = b""
    for s, blob in blobs:
        dim = 0 if s >= 256 else s
        dir_entries += struct.pack(
            "<BBBBHHII", dim, dim, 0, 0, 1, 32, len(blob), offset + len(data)
        )
        data += blob
    with open(path, "wb") as f:
        f.write(out + dir_entries + data)


if __name__ == "__main__":
    import os

    os.makedirs("assets", exist_ok=True)
    write_ico("assets/icon.ico")
    render(256).save("assets/icon.png")
    render(512).save("assets/icon-512.png")
    # RGBA mentah untuk ikon jendela (dipakai `include_bytes!` di main.rs).
    with open("assets/icon-64.rgba", "wb") as f:
        f.write(render(64).tobytes())
    for p in (
        "assets/icon.ico",
        "assets/icon.png",
        "assets/icon-512.png",
        "assets/icon-64.rgba",
    ):
        print(p, os.path.getsize(p), "bytes")
