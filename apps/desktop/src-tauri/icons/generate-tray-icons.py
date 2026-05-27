#!/usr/bin/env python3
"""Generate macOS template tray icons for Vibe Monitor.

tray.png — simplified logo from icon.png (monitor + V), see docs/plans/tray-brand-fixed-icon.md
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

BLACK = (0, 0, 0, 255)
OUT = Path(__file__).resolve().parent
LOGO_SRC = OUT / "icon.png"

CANVAS = 22
GLYPH = 19
MARGIN = (CANVAS - GLYPH) // 2
SUPERSAMPLE = 4
DRAW = CANVAS * SUPERSAMPLE

# Center crop on icon.png — drops handwritten text around the monitor
LOGO_CROP_FRAC = 0.30
LOGO_INK_THRESHOLD = 190


def canvas() -> tuple[Image.Image, ImageDraw.ImageDraw]:
    img = Image.new("RGBA", (DRAW, DRAW), (0, 0, 0, 0))
    return img, ImageDraw.Draw(img)


def to_output(img: Image.Image) -> Image.Image:
    return img.resize((CANVAS, CANVAS), Image.Resampling.LANCZOS)


def raster_logo_template(src_path: Path) -> Image.Image:
    """icon.png → black + alpha, trimmed, fitted to glyph box."""
    src = Image.open(src_path).convert("RGBA")
    w, h = src.size
    m = int(w * LOGO_CROP_FRAC)
    cropped = src.crop((m, m, w - m, h - m))
    gray = cropped.convert("L")
    ink = Image.new("RGBA", cropped.size, (0, 0, 0, 0))
    gpx, ipx = gray.load(), ink.load()
    for y in range(cropped.height):
        for x in range(cropped.width):
            if gpx[x, y] < LOGO_INK_THRESHOLD:
                ipx[x, y] = BLACK
    bbox = ink.getbbox()
    if bbox is None:
        raise ValueError(f"no ink in logo crop: {src_path}")
    ink = ink.crop(bbox)

    target = GLYPH * SUPERSAMPLE
    ink.thumbnail((target, target), Image.Resampling.LANCZOS)

    sheet = Image.new("RGBA", (DRAW, DRAW), (0, 0, 0, 0))
    ox = (DRAW - ink.width) // 2
    oy = (DRAW - ink.height) // 2
    sheet.paste(ink, (ox, oy), ink)
    return to_output(sheet)


def brand_mark_vector(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    """Fallback: thick monitor outline + solid V + power dot (matches logo layout)."""
    s = SUPERSAMPLE
    stroke = max(2, int(2.5 * s))
    draw.rounded_rectangle(
        (cx - 8 * s, cy - 7 * s, cx + 8 * s, cy + 4 * s),
        radius=2 * s,
        outline=BLACK,
        width=stroke,
    )
    draw.rounded_rectangle(
        (cx - 5 * s, cy + 4 * s, cx + 5 * s, cy + 6 * s),
        radius=1 * s,
        fill=BLACK,
    )
    draw.polygon(
        [
            (cx, cy - 5 * s),
            (cx - 5 * s, cy + 2 * s),
            (cx - 2 * s, cy + 2 * s),
            (cx, cy - 1 * s),
            (cx + 2 * s, cy + 2 * s),
            (cx + 5 * s, cy + 2 * s),
        ],
        fill=BLACK,
    )
    pr = int(1.3 * s)
    draw.ellipse(
        (cx - pr, cy + 3 * s - pr, cx + pr, cy + 3 * s + pr),
        fill=BLACK,
    )


def phase_active(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    r = (GLYPH // 2) * SUPERSAMPLE
    draw.ellipse((cx - r, cy - r, cx + r, cy + r), fill=BLACK)


def phase_waiting(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    r = (GLYPH // 2) * SUPERSAMPLE
    stroke = 2 * SUPERSAMPLE
    draw.arc((cx - r, cy - r, cx + r, cy + r), 40, 320, fill=BLACK, width=stroke)


def phase_idle(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    r = (GLYPH // 2) * SUPERSAMPLE
    stroke = max(1, int(1.5 * SUPERSAMPLE))
    draw.ellipse((cx - r, cy - r, cx + r, cy + r), outline=BLACK, width=stroke)


def phase_stopped(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    d = 5 * SUPERSAMPLE
    stroke = 2 * SUPERSAMPLE
    draw.line((cx - d, cy - d, cx + d, cy + d), fill=BLACK, width=stroke)
    draw.line((cx + d, cy - d, cx - d, cy + d), fill=BLACK, width=stroke)


def phase_unknown(draw: ImageDraw.ImageDraw, cx: int, cy: int) -> None:
    r = (GLYPH // 2) * SUPERSAMPLE
    thin = max(1, int(1.5 * SUPERSAMPLE))
    draw.ellipse((cx - r, cy - r, cx + r, cy + r), outline=BLACK, width=thin)
    dr = int(1.5 * SUPERSAMPLE)
    draw.ellipse((cx - dr, cy - dr, cx + dr, cy + dr), fill=BLACK)


PHASE_SHAPES = {
    "tray-active.png": phase_active,
    "tray-waiting.png": phase_waiting,
    "tray-idle.png": phase_idle,
    "tray-stopped.png": phase_stopped,
    "tray-unknown.png": phase_unknown,
}


def center() -> tuple[int, int]:
    c = MARGIN * SUPERSAMPLE + (GLYPH * SUPERSAMPLE) // 2
    return c, c


def write_brand() -> None:
    if LOGO_SRC.is_file():
        tray = raster_logo_template(LOGO_SRC)
        tray.save(OUT / "tray.png", optimize=True)
        print(f"wrote {OUT / 'tray.png'} (from {LOGO_SRC.name}, logo template)")
        return
    cx, cy = center()
    img, draw = canvas()
    brand_mark_vector(draw, cx, cy)
    to_output(img).save(OUT / "tray.png", optimize=True)
    print(f"wrote {OUT / 'tray.png'} (vector fallback)")


def write_phase_icons() -> None:
    cx, cy = center()
    for name, draw_fn in PHASE_SHAPES.items():
        img, draw = canvas()
        draw_fn(draw, cx, cy)
        to_output(img).save(OUT / name, optimize=True)
        print(f"wrote {OUT / name}")


def main() -> None:
    write_brand()
    write_phase_icons()


if __name__ == "__main__":
    main()
