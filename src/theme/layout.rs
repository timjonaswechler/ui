//! Dynamic layout system with responsive scaling and design token management
//!
//! This module provides a comprehensive layout system that automatically adapts
//! to different screen sizes, user preferences, and accessibility requirements
//! through dynamic scaling factors. The system ensures consistent spatial
//! relationships while providing flexibility for different deployment contexts.
//!
//! # Core Features
//!
//! - **Dynamic Scaling**: Automatic adaptation to screen density and user preferences
//! - **Responsive Design**: Values that scale proportionally across device sizes
//! - **Accessibility Support**: Configurable scaling for visual accessibility needs
//! - **Design Token System**: Semantic spacing scales with mathematical progression
//! - **Performance Optimized**: Pre-calculated values for efficient runtime performance
//!
//! # Scaling Architecture
//!
//! The layout system uses three key scaling factors:
//! - **SCALING**: Global scale factor for different screen densities and accessibility
//! - **SPACING_FACTOR**: Base multiplier for spatial relationships
//! - **FONT_SIZE_BASE**: Foundation for typography-relative measurements
//!
//! # Mathematical Foundation
//!
//! Spacing values follow a linear progression (1x, 2x, 3x, etc.) which:
//! - Provides predictable, consistent spacing relationships
//! - Scales proportionally across all screen sizes
//! - Maintains accessibility requirements at all scales
//! - Simplifies mental model for designers and developers
//!
//! # Usage Examples
//!
//! ```rust
//! use your_crate::theme::layout::UiLayout;
//!
//! let layout = UiLayout::default();
//!
//! // Spacing values automatically scale with global factors
//! let button_padding = layout.padding.base;  // 3 × SPACING_FACTOR × SCALING
//! let card_gap = layout.gap.lg;              // 4 × SPACING_FACTOR × SCALING
//! let border_radius = layout.radius.base;    // 0.375 × FONT_SIZE_BASE × SCALING
//!
//! // Values adapt to different contexts automatically
//! let responsive_margin = layout.margin.xl;  // Scales with screen size
//! ```
//!
//! # Design System Integration
//!
//! The layout system integrates with:
//! - Typography system through font-size-relative radius values
//! - Theme system for consistent visual hierarchy
//! - Accessibility system for user preference adaptation
//! - Component system for standardized spacing application
//!
//! # Performance Characteristics
//!
//! - **Compile-Time Optimization**: Values calculated once at startup
//! - **Zero Runtime Cost**: No dynamic calculations during layout
//! - **Memory Efficient**: Minimal struct size with maximum flexibility
//! - **Cache Friendly**: Predictable access patterns for layout calculations

use crate::plugin::{FONT_SIZE_BASE, SCALING, SPACING_FACTOR};
use bevy::reflect::Reflect;

/// Master layout configuration with dynamic scaling and comprehensive spacing control
///
/// UiLayout provides the central configuration for all spatial design tokens in the
/// UI system. It automatically adapts to different screen sizes, accessibility
/// requirements, and user preferences through configurable scaling factors.
///
/// # Structure Components
///
/// - **padding**: Internal spacing within components and containers
/// - **margin**: External spacing between components and layout sections
/// - **gap**: Spacing between child elements in flex and grid layouts
/// - **radius**: Border radius values for corner styling and visual hierarchy
/// - **border**: Border width values for component styling and emphasis
///
/// # Dynamic Scaling
///
/// All spacing values are calculated using global scaling factors:
/// ```rust
/// // Spacing calculation: base_value × SPACING_FACTOR × SCALING
/// let actual_padding = 3.0 * SPACING_FACTOR * SCALING;
///
/// // Radius calculation: relative_value × FONT_SIZE_BASE × SCALING
/// let actual_radius = 0.375 * FONT_SIZE_BASE * SCALING;
/// ```
///
/// # Accessibility Features
///
/// - **Scalable Values**: Automatic adaptation to user accessibility preferences
/// - **Proportional Relationships**: Maintains spatial harmony at all scales
/// - **Touch Target Support**: Ensures adequate spacing for touch interfaces
/// - **High Contrast Support**: Compatible with high contrast accessibility modes
///
/// # Examples
///
/// ```rust
/// let layout = UiLayout::default();
///
/// // Component-level spacing
/// let button_padding = layout.padding.base;
/// let input_margin = layout.margin.sm;
/// let card_radius = layout.radius.lg;
///
/// // Layout-level spacing
/// let section_gap = layout.gap.xl;
/// let container_padding = layout.padding.x2l;
/// let border_width = layout.border.base;
/// ```
#[derive(Debug, Clone, Reflect)]
pub struct UiLayout {
    /// Internal spacing within components (calculated with SPACING_FACTOR)
    pub padding: UiSpacing,
    /// External spacing between components (calculated with SPACING_FACTOR)
    pub margin: UiSpacing,
    /// Spacing between child elements in layouts (calculated with SPACING_FACTOR)
    pub gap: UiSpacing,
    /// Border radius values for corner styling (calculated with FONT_SIZE_BASE)
    pub radius: UiRadius,
    /// Border width values for component emphasis (fixed pixel values)
    pub border: UiSpacing,
}

/// Dynamic spacing scale with linear progression and responsive scaling
///
/// UiSpacing provides a comprehensive 9-step spacing scale that automatically
/// adapts to different screen sizes and accessibility requirements. The scale
/// uses a simple linear progression (1x, 2x, 3x, etc.) multiplied by global
/// scaling factors for predictable, consistent spatial relationships.
///
/// # Scaling Formula
///
/// Each spacing value is calculated as:
/// ```rust
/// actual_value = step_multiplier × SPACING_FACTOR × SCALING
/// ```
///
/// Where:
/// - `step_multiplier`: Linear progression (1.0, 2.0, 3.0, etc.)
/// - `SPACING_FACTOR`: Base spacing unit (typically 4px or 8px)
/// - `SCALING`: Global scale factor for density and accessibility
///
/// # Scale Progression (Default Configuration)
///
/// Assuming SPACING_FACTOR = 4px and SCALING = 1.0:
/// - **xs (1×)**: 4px - Minimal spacing for fine details and tight layouts
/// - **sm (2×)**: 8px - Small spacing for compact elements
/// - **base (3×)**: 12px - Standard spacing unit, foundation for most layouts
/// - **lg (4×)**: 16px - Large spacing for comfortable separation
/// - **xl (5×)**: 20px - Extra large spacing for emphasis
/// - **x2l (6×)**: 24px - Double extra large for section separation
/// - **x3l (7×)**: 28px - Triple extra large for major breaks
/// - **x4l (8×)**: 32px - Quadruple extra large for architectural spacing
/// - **x5l (9×)**: 36px - Maximum spacing for hero-level separation
///
/// # Responsive Behavior
///
/// The linear progression ensures that spatial relationships remain
/// proportional across all scaling factors:
/// - Small screens: Values scale down proportionally
/// - Large screens: Values scale up proportionally
/// - High DPI: Values scale with screen density
/// - Accessibility: Values scale with user preferences
///
/// # Usage Examples
///
/// ```rust
/// let spacing = UiSpacing::default();
///
/// // Component-level spacing (automatically scaled)
/// let button_padding = spacing.base;     // 3 × factor × scale
/// let input_margin = spacing.sm;         // 2 × factor × scale
///
/// // Layout-level spacing
/// let section_gap = spacing.xl;          // 5 × factor × scale
/// let hero_margin = spacing.x4l;         // 8 × factor × scale
///
/// // The actual pixel values depend on current scaling configuration
/// ```
///
/// # Design Benefits
///
/// - **Predictable**: Linear progression is easy to understand and apply
/// - **Scalable**: Maintains proportions at any scale factor
/// - **Consistent**: Same spatial relationships across all components
/// - **Accessible**: Automatically adapts to accessibility requirements
#[derive(Debug, Clone, Reflect, Default)]
pub struct UiSpacing {
    /// Extra small (1×) - Minimal spacing for fine details
    pub xs: f32,
    /// Small (2×) - Compact element spacing
    pub sm: f32,
    /// Base (3×) - Standard spacing unit, foundation for most layouts
    pub base: f32,
    /// Large (4×) - Comfortable separation and emphasis
    pub lg: f32,
    /// Extra large (5×) - Significant visual breaks
    pub xl: f32,
    /// Double extra large (6×) - Major section separation
    pub x2l: f32,
    /// Triple extra large (7×) - Prominent layout breaks
    pub x3l: f32,
    /// Quadruple extra large (8×) - Architectural-level spacing
    pub x4l: f32,
    /// Quintuple extra large (9×) - Maximum hero-level spacing
    pub x5l: f32,
}

/// Typography-relative border radius scale with responsive scaling
///
/// UiRadius provides a comprehensive border radius scale that automatically
/// adapts to typography changes and global scaling factors. By tying radius
/// values to the base font size, the system ensures that corner rounding
/// remains proportional to text and maintains visual harmony.
///
/// # Scaling Formula
///
/// Each radius value is calculated as:
/// ```rust
/// actual_radius = relative_multiplier × FONT_SIZE_BASE × SCALING
/// ```
///
/// Where:
/// - `relative_multiplier`: Typography-relative factor (0.125, 0.25, etc.)
/// - `FONT_SIZE_BASE`: Base font size (typically 16px)
/// - `SCALING`: Global scale factor for density and accessibility
///
/// # Scale Progression (Default Configuration)
///
/// Assuming FONT_SIZE_BASE = 16px and SCALING = 1.0:
/// - **none**: 0px - Sharp corners for technical interfaces
/// - **xs (0.125×)**: 2px - Minimal rounding for subtle softening
/// - **sm (0.25×)**: 4px - Small radius for compact elements
/// - **base (0.375×)**: 6px - Standard radius for most components
/// - **lg (0.5×)**: 8px - Large radius for prominent elements
/// - **xl (0.75×)**: 12px - Extra large radius for emphasis
/// - **x2l (1.0×)**: 16px - Double extra large, equal to font size
/// - **x3l (1.5×)**: 24px - Triple extra large for hero elements
/// - **x4l (2.0×)**: 32px - Maximum geometric radius
/// - **full**: Maximum value - Fully rounded (pills, circles)
///
/// # Typography Integration
///
/// The font-size-relative approach ensures that:
/// - Radius scales proportionally with text size changes
/// - Visual hierarchy remains consistent across different font scales
/// - Accessibility zoom maintains proper visual relationships
/// - Design system cohesion is preserved at all scales
///
/// # Usage Guidelines
///
/// ## Technical Interfaces (none, xs)
/// - Data tables and precise layouts
/// - Technical dashboards and admin interfaces
/// - Minimal design systems
///
/// ## Standard Components (sm, base, lg)
/// - Buttons, cards, and form elements
/// - Navigation and interactive components
/// - Most UI elements
///
/// ## Emphasized Elements (xl, x2l, x3l)
/// - Hero sections and featured content
/// - Modal dialogs and overlays
/// - Brand-expressive components
///
/// ## Special Cases (x4l, full)
/// - Maximum geometric rounding
/// - Badges, pills, and circular elements
///
/// # Examples
///
/// ```rust
/// let radius = UiRadius::default();
///
/// // Standard component styling (scales with font size)
/// let button_radius = radius.base;       // 0.375 × font × scale
/// let card_radius = radius.lg;           // 0.5 × font × scale
/// let input_radius = radius.sm;          // 0.25 × font × scale
///
/// // Special elements
/// let badge_radius = radius.full;        // Fully rounded
/// let hero_radius = radius.x2l;          // 1.0 × font × scale
/// ```
#[derive(Debug, Clone, Reflect, Default)]
pub struct UiRadius {
    /// No radius (0px) - Sharp corners for technical interfaces
    pub none: f32,
    /// Extra small (0.125×) - Minimal rounding relative to font size
    pub xs: f32,
    /// Small (0.25×) - Compact element radius relative to font size
    pub sm: f32,
    /// Base (0.375×) - Standard radius relative to font size
    pub base: f32,
    /// Large (0.5×) - Prominent radius relative to font size
    pub lg: f32,
    /// Extra large (0.75×) - Emphasized radius relative to font size
    pub xl: f32,
    /// Double extra large (1.0×) - Radius equal to font size
    pub x2l: f32,
    /// Triple extra large (1.5×) - Hero-level radius relative to font size
    pub x3l: f32,
    /// Quadruple extra large (2.0×) - Maximum geometric radius
    pub x4l: f32,
    /// Full radius (max value) - Completely rounded for pills and circles
    pub full: f32,
}

impl Default for UiLayout {
    /// Create the default layout configuration with dynamic scaling
    ///
    /// Constructs a UiLayout instance with all spacing and radius values
    /// calculated using the current global scaling factors. This ensures
    /// that the layout automatically adapts to different screen densities,
    /// accessibility requirements, and user preferences.
    ///
    /// # Scaling Integration
    ///
    /// The default implementation integrates three scaling systems:
    ///
    /// ## Spacing Scaling (padding, margin, gap)
    /// Uses `SPACING_FACTOR × SCALING` for proportional spatial scaling:
    /// - **SPACING_FACTOR**: Base unit size (typically 4px or 8px)
    /// - **SCALING**: Global multiplier for density and accessibility
    /// - **Linear Progression**: Simple 1×, 2×, 3× multipliers for predictability
    ///
    /// ## Typography-Relative Scaling (radius)
    /// Uses `FONT_SIZE_BASE × SCALING` for text-proportional scaling:
    /// - **FONT_SIZE_BASE**: Foundation font size (typically 16px)
    /// - **Fractional Multipliers**: 0.125×, 0.25×, etc. for fine control
    /// - **Visual Harmony**: Ensures radius scales with text size changes
    ///
    /// ## Fixed Border Scaling (border)
    /// Uses fixed pixel values with subtle progression:
    /// - **Pixel Perfect**: Crisp rendering at 1× scale
    /// - **Non-Linear Progression**: Optimized for visual hierarchy
    /// - **Design Flexibility**: Border emphasis without overwhelming content
    ///
    /// # Accessibility Features
    ///
    /// The default configuration ensures:
    /// - **Touch Target Compliance**: Adequate spacing for 44px minimum targets
    /// - **Scale Responsiveness**: Values adapt to user accessibility settings
    /// - **Proportional Relationships**: Spatial harmony maintained at all scales
    /// - **High Contrast Support**: Compatible with accessibility modes
    ///
    /// # Performance Optimization
    ///
    /// Values are calculated once at initialization:
    /// - **Compile-Time Constants**: Global factors resolved at build time when possible
    /// - **Zero Runtime Cost**: No dynamic calculations during layout operations
    /// - **Memory Efficient**: Pre-calculated values minimize computation overhead
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Automatic scaling based on global configuration
    /// let layout = UiLayout::default();
    ///
    /// // Spacing values adapt to current scaling factors
    /// let button_padding = layout.padding.base; // 3 × SPACING_FACTOR × SCALING
    /// let card_gap = layout.gap.lg;             // 4 × SPACING_FACTOR × SCALING
    ///
    /// // Radius values scale with typography
    /// let input_radius = layout.radius.sm;      // 0.25 × FONT_SIZE_BASE × SCALING
    /// let modal_radius = layout.radius.xl;      // 0.75 × FONT_SIZE_BASE × SCALING
    ///
    /// // Border values provide consistent hierarchy
    /// let subtle_border = layout.border.xs;     // 1px
    /// let emphasis_border = layout.border.lg;   // 5px
    /// ```
    fn default() -> Self {
        UiLayout {
            // Padding: Internal component spacing with linear progression
            padding: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,   // Minimal internal spacing
                sm: 2.0 * SPACING_FACTOR * SCALING,   // Compact component padding
                base: 3.0 * SPACING_FACTOR * SCALING, // Standard component padding
                lg: 4.0 * SPACING_FACTOR * SCALING,   // Comfortable component padding
                xl: 5.0 * SPACING_FACTOR * SCALING,   // Generous component padding
                x2l: 6.0 * SPACING_FACTOR * SCALING,  // Spacious component padding
                x3l: 7.0 * SPACING_FACTOR * SCALING,  // Hero-level component padding
                x4l: 8.0 * SPACING_FACTOR * SCALING,  // Maximum component padding
                x5l: 9.0 * SPACING_FACTOR * SCALING,  // Architectural component padding
            },
            // Margin: External component spacing with linear progression
            margin: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,   // Minimal external spacing
                sm: 2.0 * SPACING_FACTOR * SCALING,   // Compact component margins
                base: 3.0 * SPACING_FACTOR * SCALING, // Standard component margins
                lg: 4.0 * SPACING_FACTOR * SCALING,   // Comfortable separation
                xl: 5.0 * SPACING_FACTOR * SCALING,   // Significant separation
                x2l: 6.0 * SPACING_FACTOR * SCALING,  // Major section separation
                x3l: 7.0 * SPACING_FACTOR * SCALING,  // Prominent layout breaks
                x4l: 8.0 * SPACING_FACTOR * SCALING,  // Page-level margins
                x5l: 9.0 * SPACING_FACTOR * SCALING,  // Architectural margins
            },
            // Gap: Child element spacing with linear progression
            gap: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,   // Minimal element gaps
                sm: 2.0 * SPACING_FACTOR * SCALING,   // Compact layout gaps
                base: 3.0 * SPACING_FACTOR * SCALING, // Standard layout gaps
                lg: 4.0 * SPACING_FACTOR * SCALING,   // Comfortable layout gaps
                xl: 5.0 * SPACING_FACTOR * SCALING,   // Spacious layout gaps
                x2l: 6.0 * SPACING_FACTOR * SCALING,  // Major layout gaps
                x3l: 7.0 * SPACING_FACTOR * SCALING,  // Prominent layout gaps
                x4l: 8.0 * SPACING_FACTOR * SCALING,  // Architectural layout gaps
                x5l: 9.0 * SPACING_FACTOR * SCALING,  // Maximum layout gaps
            },
            // Radius: Typography-relative border radius with fractional progression
            radius: UiRadius {
                none: 0.0,                                  // Sharp corners
                xs: 0.125 * FONT_SIZE_BASE * SCALING,      // Minimal rounding (1/8 font size)
                sm: 0.25 * FONT_SIZE_BASE * SCALING,       // Small rounding (1/4 font size)
                base: 0.375 * FONT_SIZE_BASE * SCALING,    // Standard rounding (3/8 font size)
                lg: 0.5 * FONT_SIZE_BASE * SCALING,        // Large rounding (1/2 font size)
                xl: 0.75 * FONT_SIZE_BASE * SCALING,       // Extra large rounding (3/4 font size)
                x2l: 1.0 * FONT_SIZE_BASE * SCALING,       // Double large (equal to font size)
                x3l: 1.5 * FONT_SIZE_BASE * SCALING,       // Triple large (1.5× font size)
                x4l: 2.0 * FONT_SIZE_BASE * SCALING,       // Quadruple large (2× font size)
                full: f32::MAX,                             // Fully rounded (maximum value)
            },
            // Border: Fixed pixel values with design-optimized progression
            border: UiSpacing {
                xs: 1.0,   // Hairline border for subtle definition
                sm: 2.0,   // Thin border for clear separation
                base: 3.0, // Standard border for normal emphasis
                lg: 5.0,   // Thick border for strong emphasis
                xl: 7.0,   // Very thick border for major emphasis
                x2l: 9.0,  // Extra thick border for special cases
                x3l: 12.0, // Prominent border for hero elements
                x4l: 15.0, // Heavy border for maximum emphasis
                x5l: 19.0, // Architectural border for extreme cases
            },
        }
    }
}
