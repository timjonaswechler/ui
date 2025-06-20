use bevy::prelude::*;

use crate::components::text::{Text, TextBuilder, TextSize, TextWeight, TextColor as TextColorEnum};

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
}

impl HeadingExt for TextBuilder {
    fn as_heading(self, level: HeadingLevel) -> TextBuilder {
        match level {
            HeadingLevel::H1 => {
                self.size(TextSize::X5l).weight(TextWeight::Bold)
            },
            HeadingLevel::H2 => {
                self.size(TextSize::X3l).weight(TextWeight::Bold)
            },
            HeadingLevel::H3 => {
                self.size(TextSize::X2l).weight(TextWeight::Medium)
            },
            HeadingLevel::H4 => {
                self.size(TextSize::Xl).weight(TextWeight::Medium)
            },
            HeadingLevel::H5 => {
                self.size(TextSize::Lg).weight(TextWeight::Medium)
            },
            HeadingLevel::H6 => {
                self.size(TextSize::Base).weight(TextWeight::Medium)
            },
        }
    }
}