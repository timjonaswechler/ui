# CLAUDE.md

## Guiding Principles for AI Collaboration

When tasked with creating a new feature or component, follow the workflow defined in **`/tasks/AGENT_WORKFLOW.md`**. Key principles are:

1.  **Consistency is Key:** New components MUST follow the existing architecture (Builder Pattern, `impl Bundle` creation, Theme integration). Use completed components like `Button` (`src/components/button.rs`) as a blueprint.
2.  **Reusability First:** Before creating new logic or components, always check if existing code in `src/components/`, `src/theme/`, or `src/utilities/` can be reused or extended. Explicitly state what you plan to reuse.
3.  **Research before Planning:** Your primary sources for planning are defined in `SOURCE.md`. Always consult these to understand the design goals (Radix) and technical implementation details (Bevy).
4.  **Plan before Coding:** Do not generate code immediately. Follow the planning phase in the workflow to present an implementation plan. Wait for user approval before proceeding.

## Project Overview

This is a **Radix UI-inspired component library for Bevy 0.16**, implementing UI components based on Radix UI's design principles and API patterns. The project creates reusable UI building blocks for Bevy game development with a focus on accessibility, theming, and developer experience.

## Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Run examples
cargo run --example button          # Button component demo
cargo run --example typography      # Text and typography demo
cargo run --example colors         # Color palette demo
cargo run --example font           # Font loading demo

# Check for errors
cargo check

# Format code
cargo fmt

# Run tests (when available)
cargo test

# Run clippy for linting
cargo clippy
```

## High-Level Architecture

### Core Module Structure

- **`src/lib.rs`** - Main library exports
- **`src/plugin.rs`** - `ForgeUiPlugin` main plugin with asset loading and state management
- **`src/components/`** - UI components (Button, Text, Heading, etc.)
- **`src/theme/`** - Design system (colors, typography, layout)
- **`src/utilities/`** - Helper traits and utilities
- **`src/assets/`** - Asset management for audio, textures, etc.

### Design System Architecture

**Theme System**: Built around Radix UI's 12-step color scales with semantic color roles:
- **Accent, Gray, Red, Green, Blue, Orange, Purple, Pink, Indigo, Cyan, Yellow, Crimson** color palettes
- **Functional colors**: Surface, component, border, solid, text colors
- **Light/Dark theme** support via feature flags (`light_mode`, `dark_mode`)

**Typography System**: Complete font management with:
- **Font families**: Sans (Roboto), Serif (Noto Serif), Mono (Roboto Mono)
- **Font weights**: Light, Regular, Medium, Bold + Italic variants
- **Text sizes**: Xs to X9l responsive sizing scale
- **Text variants**: Body, Caption, Label, Display, Title

### Component Architecture

**Builder Pattern**: All components use a fluent builder API:
```rust
let button = ButtonBuilder::new("Label")
    .variant(ButtonVariant::Solid)
    .size(ButtonSize::Large)
    .text_weight(TextWeight::Bold)
    .build();
```

**Component Structure**: Components follow a consistent pattern:
- **Component struct** - Holds configuration and state
- **Builder struct** - Fluent API for construction
- **Bundle creation** - Returns `impl Bundle` for Bevy ECS
- **System integration** - Event handling, animation, interaction

### Key Components Status

**✅ Completed Components:**
- **Text** - Full typography system with theme integration (`src/components/text.rs`)
- **Heading** - Semantic H1-H6 headings (`src/components/heading.rs`)
- **Button** - Interactive button with variants, loading states (`src/components/button.rs`)
- **Portal** - Render outside component tree (`src/utilities/portal.rs`)
- **UIRoot** - Foundation container (`src/utilities/ui_root.rs`)



### Bevy Integration Patterns

**Asset Management**:
- Font assets loaded via `FontAssets` resource in startup systems
- Audio effects managed through `SfxAssets`
- Texture assets for UI elements (spinner icons, etc.)

**Event System**: Uses Bevy's Observer pattern (not legacy Events):
- `ButtonClickEvent` for button interactions
- `bevy_picking` for UI interactions (hover, click, focus)

**Animation System**: Transform-based animations for UI elements:
- Loading spinners with rotation animation
- Hover effects via color/transform changes
- State-based visual transitions

### Development Guidelines

**State Management**: Components use `UiState` enum for loading phases:
- `LoadingAssets` → `LoadingTheme` → `Ready` → `HotReload`

**System Ordering**: Update systems run in coordinated order:
- Interaction systems → Animation systems → Layout systems

**Accessibility**: All interactive components include:
- Keyboard navigation support via `Focusable`
- Proper focus management with `FocusPolicy`
- Screen reader compatibility (planned)

### Architecture Philosophy

This library **wraps Bevy's native UI components** rather than reimplementing them:
- **Node** + styling for layout components
- **Button** + **Interaction** for interactive elements
- **Text** + theme integration for typography
- **ImageNode** for graphics and icons

This approach leverages Bevy's performance while providing Radix-style APIs and design consistency.
