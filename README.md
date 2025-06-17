# Radix Themes UI

Dieses Repository enthaelt unter `radix_source` eine Sammlung von Dokumentationsdateien (*.mdx), die auf den [Radix Themes](https://www.radix-ui.com/themes) und [Radix Primitives](https://www.radix-ui.com/primitives) basieren. Sie dienen als Ausgangspunkt, um wiederverwendbare UI-Bausteine in einem eigenen Bevy 0.16 Projekt zu erstellen.


## Hinweise:
- Für Interaktionen die [Bevy Picking](https://docs.rs/bevy_picking/latest/bevy_picking/) verwenden.
-  Die Struktur der Module ist an die von Radix UI angelehnt, bei der umsetzung der Komponenten wird jedoch auf die Bevy-API und -Philosophie geachtet. Hierbei wird besonders auf die Verwendung des Builder-Patterns geachtet, um eine einfache und intuitive API zu schaffen.
- Schluss endlich soll kein unterschied zwischen den Radix Primitives und Radix Themes gemacht werden. Die Komponenten wurden nur in die jeweiligen Ordner gepackt, da dies die Struktur von Radix UI ist. 

- Anatomie der Komponenten: 
  - in einigen Fällen gibt es eine Root-Komponente, die alle Teile der Komponente enthält. In diesem Fall wird auch für jede einzelne Komponente ein Modul erstellt, das die Teile der Komponente enthält.
  - Der Builder der Komponente gibt am ende in der build-Methode die Root-Komponente zurück, die alle Teile der Komponente enthält. Als Rückgabe wird `impl Bundle` genutzt, um die Komponenten in Bevy zu verwenden.

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
