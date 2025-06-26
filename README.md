# Radix Themes UI

## Ziel und Philosophie

- **Radix-Ansatz für Bevy:**  
  Die Komponentenstruktur orientiert sich an Radix UI, wird jedoch auf die Bevy-API und -Philosophie übertragen. Besonderer Wert wird auf das Builder-Pattern gelegt, um eine intuitive und flexible API zu schaffen.
- **Keine Trennung zwischen Primitives und Themes:**  
  Die Trennung der Komponenten in Primitives und Themes dient nur der Übersicht, funktional gibt es keine Unterscheidung.
- **Interaktionen:**  
  Für interaktive Elemente wird [Bevy Picking](https://docs.rs/bevy_picking/latest/bevy_picking/) verwendet.

## Komponenten-Anatomie

- Viele Komponenten bestehen aus einer Root-Komponente, die alle Teile kapselt.
- Für jede Teilkomponente wird ein eigenes Modul erstellt.
- Der Builder gibt in der `build`-Methode die Root-Komponente als `impl Bundle` zurück, was eine einfache Integration in Bevy ermöglicht.

## Projektstruktur

├── README.md
├── Cargo.toml
├── assets
│ ├── audio
│ │ ├── music/.ogg
│ │ └── sfx/.ogg
│ └── ui
│ └── icons
│ ├── controllers/**/.png
│ └── interface/.png
├── examples
├── radix_source
│ └── *.mdx
└── src
├── lib.rs # Hauptmodul, das die Bibliothek definiert
├── components # Module für einzelne UI-Elemente
├── theme # Farbpaletten, Design-Tokens, ggf. Ladefunktionen
└── utilities # Hilfsfunktionen (z. B. Slot, Portal, Styles)
text

## Hinweise zur Asset-Verwaltung

- **Git LFS:**  
  Dieses Repository nutzt [Git Large File Storage (LFS)](https://git-lfs.com/) für große Dateien wie Musik, SFX und UI-Icons.  
  Nach dem Klonen des Repositories müssen ggf. die LFS-Dateien separat geladen werden:
  ```bash
  git lfs install
  git clone https://github.com/timjonaswechler/ui.git
  cd ui
  git lfs pull
  ```

  Die folgenden Pfade werden per LFS verwaltet:
- `assets/audio/music/*.ogg`
- `assets/audio/sfx/*.ogg`
- `assets/ui/icons/controllers/**/*.png`
- `assets/ui/icons/interface/*.png`

## Entwicklung & Zusammenarbeit

- **Branches:**  
Nutze Feature-Branches für neue Komponenten oder größere Änderungen.
- **Code Style:**  
Halte dich an die Rust- und Bevy-Konventionen. Dokumentation und Beispiele sind willkommen!
- **Issues & Pull Requests:**  
Fehler, Verbesserungsvorschläge und neue Komponenten bitte als Issue oder Pull Request einreichen.

## Weiterführende Links

- [Radix Themes](https://www.radix-ui.com/themes)
- [Radix Primitives](https://www.radix-ui.com/primitives)
- [Bevy Engine](https://bevyengine.org/)
- [Bevy Picking](https://docs.rs/bevy_picking/latest/bevy_picking/)
