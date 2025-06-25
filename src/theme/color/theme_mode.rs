use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::UiColorPalettes;

/// Global theme mode enumeration for light/dark theme switching
///
/// ThemeMode represents the two primary visual themes supported by the UI system.
/// It provides a simple, thread-safe way to manage application-wide theme state
/// through a global singleton pattern with RwLock protection.
///
/// # Design Philosophy
///
/// The theme system follows these principles:
/// - **Global State**: Single source of truth for theme across the entire application
/// - **Thread Safety**: RwLock protection allows safe concurrent access
/// - **Performance**: Lightweight enum with minimal overhead
/// - **Simplicity**: Binary choice between light and dark themes
///
/// # Themes
///
/// - **Light**: Bright theme optimized for well-lit environments and daytime use
/// - **Dark**: Dark theme optimized for low-light environments and reduced eye strain
///
/// # Thread Safety
///
/// The global theme state is protected by RwLock, allowing:
/// - Multiple concurrent readers for theme queries
/// - Exclusive write access for theme changes
/// - Panic-safe error handling with descriptive messages
///
/// # Usage Examples
///
/// ## Basic Theme Operations
/// ```rust
/// use your_crate::theme::color::theme_mode::*;
///
/// // Check current theme
/// if is_dark_mode() {
///     println!("Currently using dark theme");
/// }
///
/// // Toggle between themes
/// toggle_theme();
///
/// // Set specific theme
/// set_light_mode();
/// set_dark_mode();
/// ```
///
/// ## Getting Theme-Aware Colors
/// ```rust
/// // Get current theme palette
/// let colors = theme();
/// let background = colors.background;
/// let text = colors.text;
/// ```
///
/// # Global State Management
///
/// The theme mode is stored in a global static variable using lazy initialization:
/// - Initialized to Light mode by default
/// - Persists throughout application lifetime
/// - Accessible from any part of the codebase
/// - Thread-safe operations guaranteed by RwLock
///
/// # Performance Characteristics
///
/// - **Read Operations**: Very fast with shared lock acquisition
/// - **Write Operations**: Slightly slower due to exclusive lock requirement
/// - **Memory Usage**: Minimal - single enum value + RwLock overhead
/// - **Initialization**: Lazy - only allocated when first accessed
///
/// # Error Handling
///
/// All operations include panic-safe error handling:
/// - Read operations panic with "THEME_MODE poisoned" if lock is corrupted
/// - Write operations panic with same message for consistency
/// - Poisoning can only occur if a thread panics while holding the lock
/// - In practice, poisoning is extremely rare for simple enum operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Light theme for well-lit environments and daytime use
    Light,
    /// Dark theme for low-light environments and reduced eye strain
    Dark,
}

/// Global theme mode state with thread-safe access
///
/// This static variable maintains the application-wide theme state using lazy initialization
/// and RwLock protection. It serves as the single source of truth for theme configuration
/// across the entire application.
///
/// # Implementation Details
///
/// - **Lazy Initialization**: Allocated only when first accessed to minimize startup cost
/// - **Default Value**: Initializes to Light theme for broad compatibility
/// - **Thread Safety**: RwLock allows multiple concurrent readers, exclusive writers
/// - **Global Scope**: Accessible from any module that imports this module
///
/// # Access Patterns
///
/// - **Read Access**: Multiple threads can read simultaneously
/// - **Write Access**: Exclusive access for theme changes
/// - **Lock Duration**: Minimal - only during actual read/write operations
///
/// # Usage
///
/// Direct access to this variable is generally discouraged. Instead, use the
/// provided helper functions for better ergonomics and error handling:
/// - `theme_mode()` for reading
/// - `set_theme_mode()` for writing
/// - Specialized functions like `toggle_theme()` for common operations
pub static THEME_MODE: Lazy<RwLock<ThemeMode>> = Lazy::new(|| RwLock::new(ThemeMode::Light));

/// Get the current theme mode
///
/// Returns the current global theme mode state. This is the primary function
/// for querying the active theme throughout the application.
///
/// # Returns
///
/// The current `ThemeMode` (either Light or Dark)
///
/// # Panics
///
/// Panics with "THEME_MODE poisoned" if the RwLock has been poisoned by a
/// panicking thread. This is extremely rare in practice.
///
/// # Examples
///
/// ```rust
/// match theme_mode() {
///     ThemeMode::Light => apply_light_styling(),
///     ThemeMode::Dark => apply_dark_styling(),
/// }
/// ```
///
/// # Performance
///
/// This function is very fast as it only acquires a shared read lock,
/// allowing multiple concurrent calls without blocking.
pub fn theme_mode() -> ThemeMode {
    *THEME_MODE.read().expect("THEME_MODE poisoned")
}

/// Set the global theme mode
///
/// Updates the global theme mode to the specified value. This change affects
/// the entire application immediately and will be reflected in all subsequent
/// theme queries.
///
/// # Parameters
///
/// - `mode`: The new theme mode to set (Light or Dark)
///
/// # Panics
///
/// Panics with "THEME_MODE poisoned" if the RwLock has been poisoned by a
/// panicking thread.
///
/// # Examples
///
/// ```rust
/// // Set to dark theme
/// set_theme_mode(ThemeMode::Dark);
///
/// // Set to light theme
/// set_theme_mode(ThemeMode::Light);
/// ```
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from any thread. It acquires
/// an exclusive write lock, so it may block if other threads are reading or writing.
pub fn set_theme_mode(mode: ThemeMode) {
    *THEME_MODE.write().expect("THEME_MODE poisoned") = mode;
}
/// Toggle between light and dark theme modes
///
/// Switches the current theme to the opposite mode:
/// - Light → Dark
/// - Dark → Light
///
/// This function is commonly used for theme toggle buttons and keyboard shortcuts.
///
/// # Panics
///
/// Panics with "THEME_MODE poisoned" if the RwLock has been poisoned.
///
/// # Examples
///
/// ```rust
/// // Toggle theme (Light to Dark, or Dark to Light)
/// toggle_theme_mode();
///
/// // Can be used in event handlers
/// if keyboard.just_pressed(KeyCode::KeyT) {
///     toggle_theme_mode();
/// }
/// ```
///
/// # Atomic Operation
///
/// The toggle operation is atomic - the read and write are performed under
/// a single write lock to prevent race conditions.
pub fn toggle_theme_mode() {
    let mut mode = THEME_MODE.write().expect("THEME_MODE poisoned");
    *mode = match *mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
}
/// Check if the current theme is dark mode
///
/// Returns true if the current theme mode is Dark, false otherwise.
/// This is a convenience function for conditional styling and logic.
///
/// # Returns
///
/// `true` if current theme is Dark, `false` if Light
///
/// # Examples
///
/// ```rust
/// if is_dark_mode() {
///     apply_dark_colors();
/// } else {
///     apply_light_colors();
/// }
///
/// // Use in conditional expressions
/// let text_color = if is_dark_mode() { Color::WHITE } else { Color::BLACK };
/// ```
///
/// # Performance
///
/// Very fast operation that only requires a shared read lock.
pub fn is_dark_mode() -> bool {
    matches!(
        *THEME_MODE.read().expect("THEME_MODE poisoned"),
        ThemeMode::Dark
    )
}
/// Check if the current theme is light mode
///
/// Returns true if the current theme mode is Light, false otherwise.
/// This is a convenience function for conditional styling and logic.
///
/// # Returns
///
/// `true` if current theme is Light, `false` if Dark
///
/// # Examples
///
/// ```rust
/// if is_light_mode() {
///     use_light_theme_assets();
/// }
///
/// // Use for theme-aware component behavior
/// let shadow_opacity = if is_light_mode() { 0.2 } else { 0.8 };
/// ```
///
/// # Performance
///
/// Very fast operation that only requires a shared read lock.
pub fn is_light_mode() -> bool {
    matches!(
        *THEME_MODE.read().expect("THEME_MODE poisoned"),
        ThemeMode::Light
    )
}
/// Set the theme to light mode
///
/// Convenience function that sets the global theme to Light mode.
/// Equivalent to `set_theme_mode(ThemeMode::Light)`.
///
/// # Examples
///
/// ```rust
/// // Set light theme explicitly
/// set_light_mode();
///
/// // Use in user preference handlers
/// match user_preference {
///     "light" => set_light_mode(),
///     "dark" => set_dark_mode(),
///     _ => {} // Keep current theme
/// }
/// ```
pub fn set_light_mode() {
    set_theme_mode(ThemeMode::Light);
}
/// Set the theme to dark mode
///
/// Convenience function that sets the global theme to Dark mode.
/// Equivalent to `set_theme_mode(ThemeMode::Dark)`.
///
/// # Examples
///
/// ```rust
/// // Set dark theme explicitly
/// set_dark_mode();
///
/// // Use for automatic dark mode based on time
/// if is_nighttime() {
///     set_dark_mode();
/// }
/// ```
pub fn set_dark_mode() {
    set_theme_mode(ThemeMode::Dark);
}
/// Toggle between light and dark themes
///
/// Convenience alias for `toggle_theme_mode()`. Provides a shorter function
/// name for common theme toggle operations.
///
/// # Examples
///
/// ```rust
/// // Simple theme toggle
/// toggle_theme();
///
/// // In button click handlers
/// if theme_button.clicked() {
///     toggle_theme();
/// }
/// ```
pub fn toggle_theme() {
    toggle_theme_mode();
}
/// Get the current theme's color palette
///
/// Returns the appropriate `UiColorPalettes` instance based on the current
/// theme mode. This is the primary function for obtaining theme-aware colors
/// throughout the application.
///
/// # Returns
///
/// - `UiColorPalettes::dark_mode()` if current theme is Dark
/// - `UiColorPalettes::light_mode()` if current theme is Light
///
/// # Examples
///
/// ```rust
/// // Get current theme colors
/// let colors = theme();
/// let background = colors.background;
/// let text = colors.text;
/// let accent = colors.accent;
///
/// // Use in component styling
/// fn style_button() -> ButtonStyle {
///     let theme_colors = theme();
///     ButtonStyle {
///         background_color: theme_colors.button_background,
///         text_color: theme_colors.button_text,
///         ..default()
///     }
/// }
/// ```
///
/// # Performance
///
/// This function performs a theme mode check and returns the appropriate
/// color palette. The palette creation is typically fast as it uses
/// pre-computed color values.
///
/// # Integration
///
/// This function serves as the bridge between the theme mode system and
/// the color palette system, providing a single point of access for
/// theme-aware color selection.
pub fn theme() -> UiColorPalettes {
    if is_dark_mode() {
        UiColorPalettes::dark_mode()
    } else {
        UiColorPalettes::light_mode()
    }
}
