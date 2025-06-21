use bevy::prelude::*;

use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{TextColor as TextColorEnum, TextContrastLevel},
        typography::{FontFamily, TextSize, TextWeight},
    },
};

/// Semantic heading levels for hierarchical content structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    /// Main page title (largest)
    H1,
    /// Major section heading
    #[default]
    H2,
    /// Subsection heading
    H3,
    /// Minor subsection heading
    H4,
    /// Small heading
    H5,
    /// Smallest heading
    H6,
}

/// Heading component that provides semantic heading levels with appropriate styling
pub struct Heading;

impl Heading {
    /// Create a new heading with the specified level and content
    ///
    /// # Example
    /// ```rust
    /// use your_crate::components::Heading;
    /// 
    /// let heading = Heading::new(HeadingLevel::H1, "Main Title").build();
    /// ```
    pub fn new(level: HeadingLevel, content: impl Into<String>) -> TextBuilder {
        let content = content.into();
        
        match level {
            HeadingLevel::H1 => {
                Text::new(content)
                    .size(TextSize::X5l)      // Largest heading
                    .weight(TextWeight::Bold)
                    .color(TextColorEnum::Default)
            },
            HeadingLevel::H2 => {
                Text::new(content)
                    .size(TextSize::X3l)      // Large heading
                    .weight(TextWeight::Bold)
                    .color(TextColorEnum::Default)
            },
            HeadingLevel::H3 => {
                Text::new(content)
                    .size(TextSize::X2l)      // Medium-large heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            },
            HeadingLevel::H4 => {
                Text::new(content)
                    .size(TextSize::Xl)       // Medium heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            },
            HeadingLevel::H5 => {
                Text::new(content)
                    .size(TextSize::Lg)       // Small heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            },
            HeadingLevel::H6 => {
                Text::new(content)
                    .size(TextSize::Base)     // Smallest heading
                    .weight(TextWeight::Medium)
                    .color(TextColorEnum::Default)
            },
        }
    }

    /// Create an H1 heading (main page title)
    ///
    /// # Example
    /// ```rust
    /// let title = Heading::h1("Welcome to My App").build();
    /// ```
    pub fn h1(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H1, content)
    }

    /// Create an H2 heading (major section)
    ///
    /// # Example
    /// ```rust
    /// let section = Heading::h2("Getting Started").build();
    /// ```
    pub fn h2(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H2, content)
    }

    /// Create an H3 heading (subsection)
    ///
    /// # Example
    /// ```rust
    /// let subsection = Heading::h3("Installation").build();
    /// ```
    pub fn h3(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H3, content)
    }

    /// Create an H4 heading (minor subsection)
    ///
    /// # Example
    /// ```rust
    /// let minor = Heading::h4("Prerequisites").build();
    /// ```
    pub fn h4(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H4, content)
    }

    /// Create an H5 heading (small heading)
    ///
    /// # Example
    /// ```rust
    /// let small = Heading::h5("Notes").build();
    /// ```
    pub fn h5(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H5, content)
    }

    /// Create an H6 heading (smallest heading)
    ///
    /// # Example
    /// ```rust
    /// let smallest = Heading::h6("Details").build();
    /// ```
    pub fn h6(content: impl Into<String>) -> TextBuilder {
        Self::new(HeadingLevel::H6, content)
    }
}

/// Extension trait to add heading functionality to TextBuilder
pub trait HeadingExt {
    /// Convert this text into a heading of the specified level
    ///
    /// # Example
    /// ```rust
    /// let heading = Text::new("My Title").as_heading(HeadingLevel::H2).build();
    /// ```
    fn as_heading(self, level: HeadingLevel) -> TextBuilder;

    // Advanced text styling methods for headings
    fn heading_family(self, family: FontFamily) -> TextBuilder;
    fn heading_italic(self) -> TextBuilder;
    fn heading_align(self, align: JustifyText) -> TextBuilder;
    fn heading_center(self) -> TextBuilder;
    fn heading_right(self) -> TextBuilder;
    fn heading_on_background(self, background_color: Color) -> TextBuilder;
    fn heading_contrast_level(self, level: TextContrastLevel) -> TextBuilder;
    fn heading_high_contrast(self) -> TextBuilder;
    fn heading_accessible(self) -> TextBuilder;
    fn heading_auto_contrast(self) -> TextBuilder;
    fn heading_manual_color(self) -> TextBuilder;
    fn heading_color(self, color: TextColorEnum) -> TextBuilder;
}

impl HeadingExt for TextBuilder {
    fn as_heading(self, level: HeadingLevel) -> TextBuilder {
        let base_builder = match level {
            HeadingLevel::H1 => {
                self.size(TextSize::X5l).weight(TextWeight::Bold).high_contrast()
            },
            HeadingLevel::H2 => {
                self.size(TextSize::X3l).weight(TextWeight::Bold).high_contrast()
            },
            HeadingLevel::H3 => {
                self.size(TextSize::X2l).weight(TextWeight::Medium).high_contrast()
            },
            HeadingLevel::H4 => {
                self.size(TextSize::Xl).weight(TextWeight::Medium).high_contrast()
            },
            HeadingLevel::H5 => {
                self.size(TextSize::Lg).weight(TextWeight::Medium)
            },
            HeadingLevel::H6 => {
                self.size(TextSize::Base).weight(TextWeight::Medium)
            },
        };
        
        // Apply semantic-aware defaults for accessibility
        match level {
            HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3 => {
                base_builder.accessible() // WCAG AAA for important headings
            },
            _ => {
                base_builder.high_contrast() // WCAG AA for smaller headings
            }
        }
    }

    fn heading_family(self, family: FontFamily) -> TextBuilder {
        self.family(family)
    }

    fn heading_italic(self) -> TextBuilder {
        self.italic()
    }

    fn heading_align(self, align: JustifyText) -> TextBuilder {
        self.align(align)
    }

    fn heading_center(self) -> TextBuilder {
        self.center()
    }

    fn heading_right(self) -> TextBuilder {
        self.right()
    }

    fn heading_on_background(self, background_color: Color) -> TextBuilder {
        self.on_background(background_color)
    }

    fn heading_contrast_level(self, level: TextContrastLevel) -> TextBuilder {
        self.contrast_level(level)
    }

    fn heading_high_contrast(self) -> TextBuilder {
        self.high_contrast()
    }

    fn heading_accessible(self) -> TextBuilder {
        self.accessible()
    }

    fn heading_auto_contrast(self) -> TextBuilder {
        self.auto_contrast()
    }

    fn heading_manual_color(self) -> TextBuilder {
        self.manual_color()
    }

    fn heading_color(self, color: TextColorEnum) -> TextBuilder {
        self.color(color)
    }
}