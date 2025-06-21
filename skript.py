#!/usr/bin/env python3
"""
css_ts_to_rust_palette.py  – v3

Fix: **Kontrast‑Token ist manchmal nur „white“ / „black“** statt Hex‑Code. Das
wird jetzt erkannt und korrekt in sRGBA umgewandelt, sodass `high_contrast`
nicht mehr leer bleibt.

Weitere Details siehe Kopf­kommentar der vorherigen Version.
"""
from __future__ import annotations
import re
import sys
import pathlib
import requests
from typing import Dict, Tuple, Optional

# ——— Konstanten & Regex ———
BASE_FIELDS = [
    "base", "bg_subtle", "bg", "bg_hover", "bg_active", "line",
    "border", "border_hover", "solid", "solid_hover", "text", "text_contrast",
    "high_contrast", "surface", "indicator", "track",
]
INDEX_NAMES = BASE_FIELDS[:12]

HEX_RE = re.compile(r"^#(?P<hex>[0-9a-fA-F]{6})(?P<a>[0-9a-fA-F]{2})?$")
RGB_RE = re.compile(
    r"rgba?\(\s*(?P<r>\d{1,3})(?:[, ]+)(?P<g>\d{1,3})(?:[, ]+)(?P<b>\d{1,3})"
    r"(?:\s*[/,]\s*(?P<a>[0-9.]+)\s*)?\)",
    re.I,
)
NAMED_COLORS = {
    "white": "#ffffff",
    "black": "#000000",
    "transparent": "#00000000",
}

GITHUB_HTTP = ("http://github.com/", "https://github.com/")
RAW_PREFIX = "https://raw.githubusercontent.com/"
THEME_CSS_BASE = (
    "https://raw.githubusercontent.com/radix-ui/themes/main/"
    "packages/radix-ui-themes/src/styles/tokens/colors/{color}.css"
)

# ——— Hilfsfunktionen I/O ———

def to_raw_github(url: str) -> str:
    if url.startswith(RAW_PREFIX):
        return url
    if any(url.startswith(p) for p in GITHUB_HTTP):
        parts = url.split("github.com/")[1].split("/")
        if "blob" in parts:
            blob_idx = parts.index("blob")
            user_repo = parts[:blob_idx]
            branch_and_path = parts[blob_idx + 1 :]
            return RAW_PREFIX + "/".join(user_repo + branch_and_path)
    return url


def fetch_text(src: str) -> str:
    if src.startswith(("http://", "https://")):
        url = to_raw_github(src)
        r = requests.get(url, timeout=20, headers={"User-Agent": "css-ts-to-rust-palette"})
        r.raise_for_status()
        return r.text
    path = pathlib.Path(src)
    if not path.exists():
        sys.exit(f"Datei nicht gefunden: {path}")
    return path.read_text(encoding="utf-8")

# ——— Color Parsing ———

def parse_color(value: str) -> Optional[Tuple[float, float, float, float]]:
    value = value.strip().lower().rstrip(';')
    # Benannte Farben → Hex ersetzen
    value = NAMED_COLORS.get(value, value)

    if (m := HEX_RE.match(value)):
        hexval = m.group("hex")
        r, g, b = (int(hexval[i:i+2], 16) for i in (0, 2, 4))
        a_hex = m.group("a")
        a = int(a_hex, 16) if a_hex else 255
        return r/255, g/255, b/255, a/255

    if (m := RGB_RE.match(value)):
        r = int(m.group("r")) / 255
        g = int(m.group("g")) / 255
        b = int(m.group("b")) / 255
        a_raw = float(m.group("a")) if m.group("a") else 1.0
        a = a_raw / 255 if a_raw > 1 else a_raw
        return r, g, b, a

    return None  # unbekanntes Format


def fmt_srgba(rgba: Tuple[float, float, float, float]) -> str:
    r, g, b, a = rgba
    return f"Color::srgba({r:.7f}, {g:.7f}, {b:.7f}, {a:.7f})"

# ——— TypeScript‑Paletten (dark.ts) ———

def extract_palettes(ts_src: str) -> Dict[str, Dict[str, str]]:
    palettes: Dict[str, Dict[str, str]] = {}
    pos = 0
    while (m := re.search(r"export\s+const\s+(\w+)\s*=\s*\{", ts_src[pos:])):
        name = m.group(1)
        brace_start = pos + m.end() - 1
        brace_level, i = 1, brace_start + 1
        while i < len(ts_src) and brace_level:
            brace_level += (ts_src[i] == '{') - (ts_src[i] == '}')
            i += 1
        block = ts_src[brace_start+1:i-1]
        pos = i
        if "P3" in name:
            continue
        entries: Dict[str, str] = {}
        for line in block.splitlines():
            line = line.strip()
            if not line or line.startswith(("//", "/*")) or ':' not in line:
                continue
            key, val = (x.strip().strip("'\"") for x in line.split(':', 1))
            val = val.split(',',1)[0].strip().strip("'\"")
            entries[key] = val
        palettes[name] = entries
    return palettes


def root_name(name: str) -> str:
    for suffix in ("DarkA", "Dark", "A"):
        if name.endswith(suffix):
            return name[:-len(suffix)]
    return name


def combine_palettes(palettes: Dict[str, Dict[str, str]]) -> Dict[str, Dict[str, str]]:
    combined: Dict[str, Dict[str, str]] = {}
    for name, entries in palettes.items():
        root = root_name(name)
        is_alpha = name.endswith("A")
        data = combined.setdefault(root, {})
        for key, value in entries.items():
            if (m := re.match(r"^[a-zA-Z]+A?(\d{1,2})$", key)):
                idx = int(m.group(1)) - 1
                if 0 <= idx < 12:
                    field = INDEX_NAMES[idx] + ("_a" if is_alpha else "")
                    data[field] = value
    return combined

# ——— CSS Tokens (contrast/surface) ———

def fetch_css_tokens(color: str) -> Tuple[Optional[str], Optional[str]]:
    url = THEME_CSS_BASE.format(color=color)
    try:
        css = fetch_text(url)
    except Exception:
        return None, None

    def token(pattern: str) -> Optional[str]:
        m = re.search(pattern, css, re.I)
        if not m:
            return None
        val = m.group(1).strip()
        # eventuell trailing semicolon entfernen
        return val.rstrip(';')

    contrast = token(rf"--{color}-contrast:\s*([^;]+)")
    surface  = token(rf"--{color}-surface:\s*([^;]+)")
    return contrast, surface


def augment(combined: Dict[str, Dict[str, str]]) -> None:
    for color, data in combined.items():
        contrast, surface = fetch_css_tokens(color)
        if contrast:
            data.setdefault("high_contrast", contrast)
        if surface:
            data.setdefault("surface", surface)
        if (solid := data.get("solid")):
            data.setdefault("indicator", solid)
            data.setdefault("track", solid)

# ——— Ausgabe ———

def print_palettes(combined: Dict[str, Dict[str, str]]) -> None:
    order = BASE_FIELDS + [f"{n}_a" for n in INDEX_NAMES]
    for pal_name in sorted(combined):
        print(f"{pal_name}: UiColorPalette {{")
        for field in order:
            if field not in combined[pal_name]:
                continue
            raw = combined[pal_name][field]
            rgba = parse_color(raw)
            rendered = fmt_srgba(rgba) if rgba else raw
            print(f"    {field}: {rendered},")
        print("},\n")


# ——— Main ———

def main() -> None:
    url = "https://github.com/radix-ui/colors/blob/main/src/light.ts"
    src = fetch_text(url)
    palettes = extract_palettes(src)
    combined = combine_palettes(palettes)
    augment(combined)
    print_palettes(combined)


if __name__ == "__main__":
    main()
