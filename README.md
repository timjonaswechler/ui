# Radix Themes UI

Dieses Repository enthaelt unter `radix_source` eine Sammlung von Dokumentationsdateien (*.mdx), die auf den [Radix Themes](https://www.radix-ui.com/themes) und [Radix Primitives](https://www.radix-ui.com/primitives) basieren. Sie dienen als Ausgangspunkt, um wiederverwendbare UI-Bausteine in einem eigenen Bevy 0.16 Projekt zu erstellen.


## Hinweise:
Für Interaktionen die [Bevy Picking](https://docs.rs/bevy_picking/latest/bevy_picking/) verwenden.

## aktuelle Struktur:
├── README.md
├── Cargo.toml
├── assets
├── examples
├── radix_source
│   ├── primitives
│   │   ├── components
│   │   ├── guides
│   │   ├── overview
│   │   └── utilities
│   └── themes
│       ├── components
│       ├── overview
│       └── theme
└── src
    ├── lib.rs           Hauptmodul, das die Bibliothek definiert
    ├── components       Module für einzelne UI-Elemente
    ├── theme            Farbpaletten, Design-Tokens, ggf. Ladefunktionen
    └── utilities        Hilfsfunktionen (z. B. Slot, Portal, Styles)
