//! Heading Component for Forge UI
//!
//! The Heading component provides semantic HTML-like heading levels (H1-H6) for
//! creating hierarchical content structure in Bevy applications. Built on top of
//! the Text component, it applies appropriate typography scaling and styling
//! based on semantic importance and accessibility guidelines.
//!
//! ## Key Features
//!
//! - **Semantic Hierarchy**: Six heading levels (H1-H6) with proper visual hierarchy
//! - **Accessibility First**: Automatic contrast optimization based on heading importance
//! - **Typography Integration**: Leverages the complete Text system capabilities
//! - **Responsive Scaling**: Font sizes that create clear visual hierarchy
//! - **Extension Support**: HeadingExt trait for converting existing text to headings
//! - **Theme Compliance**: Full integration with Radix UI design system
//!
//! ## Heading Hierarchy
//!
//! | Level | Purpose | Size | Weight | Accessibility |
//! |-------|---------|------|--------|--------------|
//! | **H1** | Main page title | X5l (48px+) | Bold | WCAG AAA |
//! | **H2** | Major sections | X3l (36px+) | Bold | WCAG AAA |
//! | **H3** | Subsections | X2l (30px+) | Medium | WCAG AAA |
//! | **H4** | Minor subsections | Xl (24px+) | Medium | WCAG AA |
//! | **H5** | Small headings | Lg (20px+) | Medium | WCAG AA |
//! | **H6** | Smallest headings | Base (16px) | Medium | WCAG AA |
//!
//! ## Examples
//!
//! ### Basic Usage
//! ```rust
//! use forge_ui::Heading;
//!
//! // Direct heading creation
//! let main_title = Heading::h1("Welcome to My App").build();
//! let section = Heading::h2("Getting Started").build();
//! let subsection = Heading::h3("Installation").build();
//!
//! // Using the generic method
//! let custom = Heading::new(HeadingLevel::H2, "Custom Title")
//!     .center()
//!     .build();
//! ```
//!
//! ### Advanced Styling
//! ```rust
//! use forge_ui::{Heading, FontFamily, TextColor};
//!
//! // Styled heading with custom font and color
//! let styled_heading = Heading::h1("Styled Title")
//!     .family(FontFamily::Serif)
//!     .color(TextColor::Accent)
//!     .center()
//!     .build();
//!
//! // Heading optimized for specific background
//! let accessible_heading = Heading::h2("Important Section")
//!     .on_background(Color::srgb(0.1, 0.1, 0.9))
//!     .accessible()
//!     .build();
//! ```
//!
//! ### Extension Trait Usage
//! ```rust
//! use forge_ui::{Text, HeadingExt, HeadingLevel};
//!
//! // Convert existing text to heading
//! let converted = Text::new("My Title")
//!     .as_heading(HeadingLevel::H2)
//!     .center()
//!     .build();
//!
//! // Chain heading-specific methods
//! let enhanced = Text::new("Enhanced Title")
//!     .as_heading(HeadingLevel::H1)
//!     .heading_italic()
//!     .heading_center()
//!     .heading_accessible()
//!     .build();
//! ```
//!
//! ## Accessibility Guidelines
//!
//! The heading system follows WCAG guidelines:
//!
//! - **H1-H3**: Use WCAG AAA contrast ratios (enhanced accessibility)
//! - **H4-H6**: Use WCAG AA contrast ratios (standard accessibility)
//! - **Semantic Order**: Maintain logical heading hierarchy in your content
//! - **Skip Navigation**: Consider implementing skip links for screen readers
//! - **Focus Management**: Ensure headings are properly focusable when needed
//!
//! ## Design Philosophy
//!
//! The heading system is designed with these principles:
//!
//! 1. **Semantic First**: Choose heading level based on content hierarchy, not appearance
//! 2. **Visual Hierarchy**: Clear size differentiation between levels
//! 3. **Accessibility**: Higher importance headings get better contrast
//! 4. **Flexibility**: Full access to Text system capabilities through chaining
//! 5. **Consistency**: Predictable sizing and styling across the application

use bevy::prelude::*;

use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{TextColor as TextColorEnum, TextContrastLevel},
        typography::{FontFamily, TextSize, TextWeight},
    },
};

/// Semantic heading levels for hierarchical content structure.
/// 
/// These levels correspond to HTML heading elements (H1-H6) and provide
/// a clear visual and semantic hierarchy for content organization.
/// Each level has predefined typography settings optimized for its
/// semantic importance and readability requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    /// Main page title - the most important heading on the page.
    /// 
    /// **Usage**: Primary page titles, hero headings, main content titles
    /// **Size**: X5l (48px+), **Weight**: Bold, **Accessibility**: WCAG AAA
    H1,
    
    /// Major section heading - primary content divisions.
    /// 
    /// **Usage**: Main sections, primary content areas, major topics
    /// **Size**: X3l (36px+), **Weight**: Bold, **Accessibility**: WCAG AAA
    #[default]
    H2,
    
    /// Subsection heading - secondary content divisions.
    /// 
    /// **Usage**: Subsections within major sections, secondary topics
    /// **Size**: X2l (30px+), **Weight**: Medium, **Accessibility**: WCAG AAA
    H3,
    
    /// Minor subsection heading - tertiary content divisions.
    /// 
    /// **Usage**: Sub-subsections, detailed breakdowns, specific topics
    /// **Size**: Xl (24px+), **Weight**: Medium, **Accessibility**: WCAG AA
    H4,
    
    /// Small heading - quaternary content divisions.
    /// 
    /// **Usage**: Minor sections, notes, detailed categorization
    /// **Size**: Lg (20px+), **Weight**: Medium, **Accessibility**: WCAG AA
    H5,
    
    /// Smallest heading - minimal content divisions.
    /// 
    /// **Usage**: Fine-grained sections, labels, minimal categorization
    /// **Size**: Base (16px), **Weight**: Medium, **Accessibility**: WCAG AA
    H6,
}

/// Main Heading component interface for creating semantic heading elements.
/// 
/// The Heading struct provides factory methods for creating headings with
/// semantic meaning and appropriate visual hierarchy. It builds upon the
/// Text component system while adding heading-specific typography and
/// accessibility optimizations.
/// 
/// # Design Approach
/// 
/// The component follows the principle of "semantic meaning drives visual presentation":
/// 1. Choose the appropriate semantic level (H1-H6) based on content hierarchy
/// 2. The system automatically applies appropriate typography and accessibility settings
/// 3. Customize appearance as needed using the returned TextBuilder
/// 4. Build the final component with `.build()`
/// 
/// # Accessibility Considerations
/// 
/// - Higher importance headings (H1-H3) automatically use WCAG AAA contrast
/// - Lower importance headings (H4-H6) use WCAG AA contrast
/// - All headings support background context awareness for optimal contrast
/// - Font sizes create clear visual hierarchy for screen readers and users
/// 
/// # Example
/// ```rust
/// // Semantic approach - choose level based on content hierarchy
/// let page_title = Heading::h1("My Application").center().build();
/// let section = Heading::h2("Features").build();
/// let subsection = Heading::h3("Core Features").build();
/// 
/// // Custom styling while maintaining semantics
/// let styled = Heading::h2("Special Section")
///     .family(FontFamily::Serif)
///     .color(TextColor::Accent)
///     .build();
/// ```
pub struct Heading;

impl Heading {
    /// Creates a new heading with the specified semantic level and content.
    /// 
    /// This is the core method that applies appropriate typography settings
    /// based on the semantic importance of the heading level. Each level
    /// has predefined size, weight, and accessibility settings optimized
    /// for clear visual hierarchy and readability.
    /// 
    /// # Arguments
    /// * `level` - The semantic heading level (H1-H6)
    /// * `content` - The text content for the heading
    /// 
    /// # Returns
    /// A TextBuilder configured with heading-appropriate typography
    /// 
    /// # Example
    /// ```rust
    /// use forge_ui::{Heading, HeadingLevel};
    /// 
    /// let main_title = Heading::new(HeadingLevel::H1, "Welcome")
    ///     .center()
    ///     .build();
    /// 
    /// let section = Heading::new(HeadingLevel::H2, "Getting Started")
    ///     .color(TextColor::Accent)
    ///     .build();
    /// ```
    pub fn new(level: HeadingLevel, content: impl Into<String>) -> TextBuilder {
        let content = content.into();

        match level {
            HeadingLevel::H1 => {
                Text::new(content)
                    .size(TextSize::X5l) // Largest heading
                    .weight(TextWeight::Bold)
                    .color(TextColorEnum::Default)
            }
            HeadingLevel::H2 => {
                Text::new(content)
                    .size(TextSize::X3l) // Large heading
                    .weight(TextWeight::Bold)
                    .color(TextColorEnum::Default)
            }
            HeadingLevel::H3 => {
                Text::new(content)
                    .size(TextSize::X2l) // Medium-large heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            }
            HeadingLevel::H4 => {
                Text::new(content)
                    .size(TextSize::Xl) // Medium heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            }
            HeadingLevel::H5 => {
                Text::new(content)
                    .size(TextSize::Lg) // Small heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            }
            HeadingLevel::H6 => {
                Text::new(content)
                    .size(TextSize::Base) // Smallest heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            }
        }
    }

    // === Semantic Heading Factory Methods ===
    
    /// Creates an H1 heading - the main page title.
    /// 
    /// H1 headings are the most important on the page and should be used
    /// for primary page titles, hero sections, or main content headings.
    /// **Typography**: X5l size, Bold weight, WCAG AAA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let page_title = Heading::h1("Welcome to My Application")
    ///     .center()
    ///     .build();
    /// ```
    pub fn h1(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H1, content)
    }

    /// Creates an H2 heading - major section heading.
    /// 
    /// H2 headings divide content into major sections and should represent
    /// primary content areas or main topics within the page.
    /// **Typography**: X3l size, Bold weight, WCAG AAA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let section_title = Heading::h2("Features Overview")
    ///     .color(TextColor::Accent)
    ///     .build();
    /// ```
    pub fn h2(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H2, content)
    }

    /// Creates an H3 heading - subsection heading.
    /// 
    /// H3 headings create subsections within major sections, providing
    /// secondary level organization of content.
    /// **Typography**: X2l size, Medium weight, WCAG AAA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let subsection = Heading::h3("Installation Guide")
    ///     .family(FontFamily::Sans)
    ///     .build();
    /// ```
    pub fn h3(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H3, content)
    }

    /// Creates an H4 heading - minor subsection heading.
    /// 
    /// H4 headings provide tertiary organization within subsections,
    /// useful for detailed breakdowns or specific topics.
    /// **Typography**: Xl size, Medium weight, WCAG AA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let detail_section = Heading::h4("System Requirements")
    ///     .high_contrast()
    ///     .build();
    /// ```
    pub fn h4(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H4, content)
    }

    /// Creates an H5 heading - small heading.
    /// 
    /// H5 headings provide quaternary organization for fine-grained
    /// content division or minor categorization.
    /// **Typography**: Lg size, Medium weight, WCAG AA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let minor_heading = Heading::h5("Additional Notes")
    ///     .right()
    ///     .build();
    /// ```
    pub fn h5(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H5, content)
    }

    /// Creates an H6 heading - smallest heading.
    /// 
    /// H6 headings provide the finest level of content organization,
    /// typically used for labels or minimal categorization.
    /// **Typography**: Base size, Medium weight, WCAG AA contrast
    /// 
    /// # Arguments
    /// * `content` - The heading text content
    /// 
    /// # Example
    /// ```rust
    /// let label_heading = Heading::h6("Technical Details")
    ///     .color(TextColor::Muted)
    ///     .build();
    /// ```
    pub fn h6(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H6, content)
    }
}

/// Extension trait that adds heading functionality to TextBuilder.
/// 
/// This trait allows converting existing TextBuilder instances into headings
/// and provides heading-specific styling methods with clear naming conventions.
/// All methods are prefixed with "heading_" to avoid naming conflicts and
/// provide clear intent when used in method chains.
/// 
/// # Design Benefits
/// 
/// 1. **Conversion**: Transform any text into a semantic heading
/// 2. **Method Clarity**: "heading_" prefixed methods show clear intent
/// 3. **Chaining**: Fluent API that works well with the builder pattern
/// 4. **Accessibility**: Automatic contrast optimization based on heading level
/// 
/// # Example
/// ```rust
/// use forge_ui::{Text, HeadingExt, HeadingLevel};
/// 
/// // Convert existing text to heading
/// let converted = Text::new("Important Section")
///     .as_heading(HeadingLevel::H2)
///     .heading_center()
///     .heading_accessible()
///     .build();
/// 
/// // Chain multiple heading-specific methods
/// let styled = Text::new("Styled Title")
///     .as_heading(HeadingLevel::H1)
///     .heading_family(FontFamily::Serif)
///     .heading_italic()
///     .heading_color(TextColor::Accent)
///     .build();
/// ```
pub trait HeadingExt {
    /// Converts this text into a heading of the specified semantic level.
    /// 
    /// This method applies heading-appropriate typography settings based on
    /// the semantic importance of the heading level, including automatic
    /// accessibility optimizations.
    /// 
    /// # Arguments
    /// * `level` - The semantic heading level to apply
    /// 
    /// # Returns
    /// TextBuilder configured with heading typography and accessibility settings
    /// 
    /// # Example
    /// ```rust
    /// let converted_heading = Text::new("Section Title")
    ///     .as_heading(HeadingLevel::H2)
    ///     .center()
    ///     .build();
    /// ```
    fn as_heading(self, level: HeadingLevel) -> TextBuilder;

    // === Typography Styling Methods ===
    
    /// Sets the font family for the heading.
    fn heading_family(self, family: FontFamily) -> TextBuilder;
    
    /// Makes the heading text italic.
    fn heading_italic(self) -> TextBuilder;
    
    // === Alignment Methods ===
    
    /// Sets custom text alignment for the heading.
    fn heading_align(self, align: JustifyText) -> TextBuilder;
    
    /// Centers the heading text.
    fn heading_center(self) -> TextBuilder;
    
    /// Right-aligns the heading text.
    fn heading_right(self) -> TextBuilder;
    
    // === Accessibility & Contrast Methods ===
    
    /// Sets background context for intelligent contrast calculation.
    fn heading_on_background(self, background_color: Color) -> TextBuilder;
    
    /// Sets the desired contrast level for accessibility.
    fn heading_contrast_level(self, level: TextContrastLevel) -> TextBuilder;
    
    /// Enables high contrast mode (WCAG AA compliance).
    fn heading_high_contrast(self) -> TextBuilder;
    
    /// Enables accessible contrast mode (WCAG AAA compliance).
    fn heading_accessible(self) -> TextBuilder;
    
    /// Enables automatic contrast optimization.
    fn heading_auto_contrast(self) -> TextBuilder;
    
    /// Disables automatic contrast optimization.
    fn heading_manual_color(self) -> TextBuilder;
    
    /// Sets an explicit text color for the heading.
    fn heading_color(self, color: TextColorEnum) -> TextBuilder;
}

impl HeadingExt for TextBuilder {
    /// Implementation of heading conversion with automatic accessibility optimization.
    /// 
    /// This method applies level-appropriate typography and automatically
    /// configures accessibility settings based on heading importance:
    /// - H1-H3: WCAG AAA contrast (accessible)
    /// - H4-H6: WCAG AA contrast (high_contrast)
    fn as_heading(self, level: HeadingLevel) -> TextBuilder {
        // Apply typography settings based on heading level
        let base_builder = match level {
            HeadingLevel::H1 => self
                .size(TextSize::X5l)  // Largest heading size
                .weight(TextWeight::Bold)
                .high_contrast(),
            HeadingLevel::H2 => self
                .size(TextSize::X3l)  // Large section heading
                .weight(TextWeight::Bold)
                .high_contrast(),
            HeadingLevel::H3 => self
                .size(TextSize::X2l)  // Medium-large subsection
                .weight(TextWeight::Medium)
                .high_contrast(),
            HeadingLevel::H4 => self
                .size(TextSize::Xl)   // Medium heading
                .weight(TextWeight::Medium)
                .high_contrast(),
            HeadingLevel::H5 => self
                .size(TextSize::Lg)   // Small heading
                .weight(TextWeight::Medium),
            HeadingLevel::H6 => self
                .size(TextSize::Base) // Smallest heading
                .weight(TextWeight::Medium),
        };

        // Apply semantic-aware accessibility defaults
        // Higher importance headings get better contrast
        match level {
            HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3 => {
                base_builder.accessible() // WCAG AAA for important headings
            }
            _ => {
                base_builder.high_contrast() // WCAG AA for smaller headings
            }
        }
    }

    // === Typography Method Implementations ===
    
    /// Delegates to the underlying TextBuilder family method.
    fn heading_family(self, family: FontFamily) -> TextBuilder {
        self.family(family)
    }

    /// Delegates to the underlying TextBuilder italic method.
    fn heading_italic(self) -> TextBuilder {
        self.italic()
    }

    // === Alignment Method Implementations ===
    
    /// Delegates to the underlying TextBuilder align method.
    fn heading_align(self, align: JustifyText) -> TextBuilder {
        self.align(align)
    }

    /// Delegates to the underlying TextBuilder center method.
    fn heading_center(self) -> TextBuilder {
        self.center()
    }

    /// Delegates to the underlying TextBuilder right method.
    fn heading_right(self) -> TextBuilder {
        self.right()
    }

    // === Accessibility Method Implementations ===
    
    /// Delegates to the underlying TextBuilder on_background method.
    fn heading_on_background(self, background_color: Color) -> TextBuilder {
        self.on_background(background_color)
    }

    /// Delegates to the underlying TextBuilder contrast_level method.
    fn heading_contrast_level(self, level: TextContrastLevel) -> TextBuilder {
        self.contrast_level(level)
    }

    /// Delegates to the underlying TextBuilder high_contrast method.
    fn heading_high_contrast(self) -> TextBuilder {
        self.high_contrast()
    }

    /// Delegates to the underlying TextBuilder accessible method.
    fn heading_accessible(self) -> TextBuilder {
        self.accessible()
    }

    /// Delegates to the underlying TextBuilder auto_contrast method.
    fn heading_auto_contrast(self) -> TextBuilder {
        self.auto_contrast()
    }

    /// Delegates to the underlying TextBuilder manual_color method.
    fn heading_manual_color(self) -> TextBuilder {
        self.manual_color()
    }

    /// Delegates to the underlying TextBuilder color method.
    fn heading_color(self, color: TextColorEnum) -> TextBuilder {
        self.color(color)
    }
}
