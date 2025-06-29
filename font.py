#!/usr/bin/env python3

from pathlib import Path
import re
import sys

# ------------------ Konfiguration ------------------
INPUT_FILE = "assets/ui/icons/controllers/xbox_series/kenney_input_xbox_series_map.txt"          # ggf. per CLI überschreiben
OUTPUT_FILE = INPUT_FILE + "output.txt"                 # None ⇒ nur stdout; Pfad-String ⇒ Datei
# ---------------------------------------------------

_CAMEL_RE = re.compile(r"[_\- ]+([A-Za-z0-9])")  # findet Trenner + Folgezeichen

def slug_to_camel(slug: str) -> str:
    """
    keyboard_0_outline  -> Keyboard0Outline
    zoom-out            -> ZoomOut
    """
    # erster Buchstabe groß …
    camel = slug[:1].upper() + slug[1:]
    # … Buchstaben/Zahlen nach _- oder Leerzeichen groß, Trenner entfernen
    return _CAMEL_RE.sub(lambda m: m.group(1).upper(), camel)

def parse_line(line: str):
    """
    Zerlegt 'keyboard: U+E000' in ('keyboard', 'e000').
    Gibt None zurück, falls die Zeile nicht passt oder leer ist.
    """
    line = line.strip()
    if not line or ":" not in line:
        return None
    name_part, code_part = map(str.strip, line.split(":", 1))
    match = re.fullmatch(r"U\+([0-9A-Fa-f]{4,6})", code_part)
    if not match:
        return None
    return name_part, match.group(1).lower()

def main(path: str | Path = INPUT_FILE):
    src = Path(path)
    out_lines: list[str] = []

    with src.open(encoding="utf-8") as f:
        for raw in f:
            parsed = parse_line(raw)
            if not parsed:
                continue  # Zeile ignorieren
            slug, hex_code = parsed
            camel = slug_to_camel(slug)
            out_lines.append(f'pub const {camel} : &str = "\\u{{{hex_code}}}";')

    # Ausgabe
    if OUTPUT_FILE:
        Path(OUTPUT_FILE).write_text("\n".join(out_lines), encoding="utf-8")
    else:
        print("\n".join(out_lines))

if __name__ == "__main__":
    # file-Pfad auch via CLI nutzbar:  python txt_icons_to_rust_strings.py myfile.txt
    main(sys.argv[1] if len(sys.argv) > 1 else INPUT_FILE)
