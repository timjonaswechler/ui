//! Button Component for Forge UI
//!
//! The Button component is a fundamental interactive element that allows users to trigger
//! actions, submit forms, navigate through applications, and perform various user interactions.
//! It provides a comprehensive API for customization while maintaining accessibility and
//! modern design principles.
//!
//! ## Features
//!
//! - **4 Visual Variants**: Solid, Soft, Outline, and Ghost styles for different UI hierarchies
//! - **3 Size Options**: Small, Default, and Large to fit various interface needs
//! - **Theme Integration**: Full support for Radix UI color palette system
//! - **Interactive States**: Normal, Hover, Active, Disabled, and Loading states
//! - **High Contrast Mode**: Enhanced accessibility with increased contrast
//! - **Flexible Border Radius**: From sharp corners to pill-shaped buttons
//! - **Text Integration**: Built-in text support with comprehensive typography options
//! - **Event System**: Built-in click handling and custom event support
//! - **Animations**: Smooth state transitions and loading indicators
//! - **Accessibility**: ARIA attributes and keyboard navigation support
//!
//! ## Examples
//!
//! ### Basic Usage
//! ```rust
//! # use ui::components::button::Button;
//! # use ui::theme::color::accent_palette;
//!
//! // Simple primary action button
//! let submit_button = Button::builder("submit")
//!     .text("Submit Form")
//!     .solid()
//!     .color(accent_palette())
//!     .build();
//!
//! // Secondary action button  
//! # use ui::theme::color::theme;
//! let cancel_button = Button::builder("cancel")
//!     .text("Cancel")
//!     .soft()
//!     .color(theme().gray)
//!     .build();
//! ```
//!
//! ### Advanced Configuration
//! ```rust
//! # use ui::components::button::{Button, ButtonVariant, ButtonSize, ButtonRadius};
//! # use ui::theme::color::theme;
//!
//! // Loading button with custom styling
//! let loading_button = Button::builder("upload")
//!     .text("Upload File")
//!     .variant(ButtonVariant::Solid)
//!     .size(ButtonSize::Large)
//!     .color(theme().blue)
//!     .loading()
//!     .radius(ButtonRadius::Large)
//!     .build();
//! ```
//!
//! ## Variant Guidelines
//!
//! - **Solid**: Use for primary actions (submit, save, continue)
//! - **Soft**: Use for secondary actions (edit, view details)
//! - **Outline**: Use for tertiary actions (cancel, back)
//! - **Ghost**: Use for subtle actions (close, minimize)
//!
//! ## Size Guidelines
//!
//! - **Small**: Compact interfaces, toolbars, table actions
//! - **Default**: Most common use cases, forms, dialogs
//! - **Large**: Call-to-action buttons, mobile interfaces, hero sections
//!
//! ## Accessibility
//!
//! - Use `high_contrast()` for better visibility
//! - Provide meaningful text labels
//! - Consider `disabled()` state for unavailable actions
//! - Use `loading()` state for async operations
//! - Ensure sufficient color contrast ratios

pub mod animations;
pub mod builder;
pub mod core;
pub mod events;
pub mod interactions;
pub mod styling;

pub use animations::*;
pub use builder::*;
pub use core::*;
pub use events::*;
pub use interactions::*;
pub use styling::*;
