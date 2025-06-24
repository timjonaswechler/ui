#!/usr/bin/env python3
"""
SVG‑Atlas‑Generator mit Mapping‑TXT
====================================

Erstellt pro Kategorie und Icon‑Größe einen Texture‑Atlas **und** eine
entsprechende Mapping‑Datei, die für jedes Original‑SVG den Index sowie
Grid‑ und Pixel‑Positionen im Atlas enthält.

Quelle:
* `controll/source/<Kategorie>/Vector/*.svg`
* `icons/source/<Kategorie>/Vector/*.svg`

Ausgaben:
```
controll/<slug>/texture_atlas_<COLS>x<ROWS>_<SIZE>px.png
controll/<slug>/icon_mapping_<SIZE>px.txt
icons/<slug>/texture_atlas_<COLS>x<ROWS>_<SIZE>px.png
icons/<slug>/icon_mapping_<SIZE>px.txt
```

Voraussetzungen: Python ≥ 3.9 · Pillow ≥ 10 · CairoSVG ≥ 2.7
"""

from __future__ import annotations

import io
import math
import os
import re
import sys
from pathlib import Path
from typing import Iterable, List

import cairosvg
from PIL import Image

# ------------------------------------------------------------
# Konfiguration
# ------------------------------------------------------------

ICON_SIZES: List[int] = [16, 24, 32, 64]   # px
GRID_COLS: int = 20                        # Spalten im Atlas
OVERSAMPLE: int = 4                        # Supersampling‑Faktor

# SVG‑Suchpfade
SOURCE_ROOTS: List[Path] = [
    Path("controll/source"),
    Path("icons/source"),
]

# ------------------------------------------------------------
# Utility‑Funktionen
# ------------------------------------------------------------

def slugify(name: str) -> str:
    """Konvertiert Ordnernamen → slug (z. B. "Nintendo WiiU" → "nintendo_wiiu")."""
    slug = re.sub(r"[&/]", " ", name).lower().strip()
    slug = re.sub(r"[^a-z0-9]+", "_", slug)
    return slug.strip("_")


def svg_to_white_png(svg_path: Path, size: int, oversample: int = OVERSAMPLE) -> Image.Image:
    """Rendert SVG monochrom‑weiß zu RGBA‑PNG der Zielgröße."""
    png_bytes = cairosvg.svg2png(url=str(svg_path), scale=oversample)
    img = Image.open(io.BytesIO(png_bytes)).convert("RGBA")
    img = img.resize((size, size), Image.Resampling.LANCZOS)

    pix = img.load()
    for y in range(img.height):
        for x in range(img.width):
            r, g, b, a = pix[x, y]
            if a:
                pix[x, y] = (255, 255, 255, a)
    return img


def _pretty(path: Path) -> str:
    """Relative Pfadangabe (Fallback: absolut)."""
    try:
        return str(path.resolve().relative_to(Path.cwd()))
    except ValueError:
        return str(path)

# ------------------------------------------------------------
# Atlas‑Erstellung
# ------------------------------------------------------------

def create_texture_atlas(svg_folder: Path, output_folder: Path, icon_size: int) -> None:
    """Erzeugt einen Atlas & Mapping‑TXT (ID, Name, Col, Row)"""
    svg_files = sorted(svg_folder.glob("*.svg"))
    if not svg_files:
        print(f"⚠️  [skip] Keine SVGs in {svg_folder}")
        return

    total_icons = len(svg_files)
    grid_rows = math.ceil(total_icons / GRID_COLS)
    atlas_w, atlas_h = GRID_COLS * icon_size, grid_rows * icon_size

    atlas = Image.new("RGBA", (atlas_w, atlas_h), (0, 0, 0, 0))
    mapping_lines: list[str] = []

    for idx, svg_file in enumerate(svg_files):
        col, row = idx % GRID_COLS, idx // GRID_COLS
        x, y = col * icon_size, row * icon_size
        icon_img = svg_to_white_png(svg_file, icon_size)
        atlas.paste(icon_img, (x, y), icon_img)

        mapping_lines.append(f"{idx:3d}	{svg_file.stem}	{col}	{row}\n")

    output_folder.mkdir(parents=True, exist_ok=True)

    # Atlas speichern
    atlas_path = output_folder / f"texture_atlas_{GRID_COLS}x{grid_rows}_{icon_size}px.png"
    atlas.save(atlas_path)

    # Mapping speicher
    map_path = output_folder / f"icon_mapping_{icon_size}px.txt"
    with map_path.open("w", encoding="utf-8") as fh:
        fh.write("# Texture Atlas Mapping\n")
        fh.write(f"# Auflösung (px): {icon_size}\n")
        fh.write(f"# Spalten: {GRID_COLS}  Zeilen: {grid_rows}\n")
        fh.write("# Format: ID	Name	Col	Row\n")
        fh.write("".join(mapping_lines))

    print(f"✅ Atlas {icon_size}px → {_pretty(atlas_path)}  ✏️ Mapping → {_pretty(map_path)}")



# ------------------------------------------------------------
# Pfad‑Hilfen & Generatoren
# ------------------------------------------------------------

def iter_vector_folders(root: Path) -> Iterable[Path]:
    for dirpath, dirnames, _ in os.walk(root):
        path = Path(dirpath)
        if path.name.lower() == "vector":
            yield path


def project_root_from_vector(vector_dir: Path) -> Path:
    for parent in vector_dir.parents:
        if parent.name == "source":
            return parent.parent
    return vector_dir.parents[-1]

# ------------------------------------------------------------
# Main
# ------------------------------------------------------------

def main() -> None:
    for src_root in SOURCE_ROOTS:
        if not src_root.exists():
            continue
        for vector_dir in iter_vector_folders(src_root):
            category_name = vector_dir.parent.name
            slug = slugify(category_name)
            project_root = project_root_from_vector(vector_dir)
            atlas_out = project_root / slug

            print(f"\n=== {project_root.name.upper()}: {category_name} ===")
            for size in ICON_SIZES:
                create_texture_atlas(vector_dir, atlas_out, size)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit("⏹️  Abbruch durch Nutzer")
