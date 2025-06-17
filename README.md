# Radix Themes for Bevy

This repository contains a Rust crate that brings Radix UI styles and components to the [Bevy](https://bevyengine.org/) game engine. The goal is to provide ready-made building blocks so you can quickly assemble modern user interfaces in your Bevy projects.

## Repository Structure

- `src/components` – reusable Bevy UI components styled with Radix
- `src/styles` – CSS and style definitions ported from Radix
- `src/utilities` – helper utilities used by the components

## Usage

Add the crate to your `Cargo.toml`:

```toml
radix_bevy = { path = "." }
```

Then import the components and styles in your Bevy app:

```rust
use radix_bevy::prelude::*;

fn setup_ui(mut commands: Commands) {
    commands.spawn(Button::new("Click me"));
}
```

Make sure to include the provided styles by loading them during startup. Explore the modules under `src` for more customization options.
