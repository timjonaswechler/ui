# Typography Examples

This document shows practical examples of using the Text and Heading components in your Bevy UI application.

## Text Component Examples

### Basic Text Usage

```rust
use your_ui_crate::components::text::{Text, TextSize, TextWeight, TextColor, FontFamily};
use your_ui_crate::utilities::ComponentBuilder;

// Simple text
commands.spawn(
    Text::new("Hello World").build()
);

// Styled text with multiple properties
commands.spawn(
    Text::new("Important Message")
        .size(TextSize::Lg)
        .weight(TextWeight::Bold)
        .color(TextColor::Error)
        .center()
        .build()
);

// Italic text
commands.spawn(
    Text::new("Emphasized text")
        .italic()
        .build()
);
```

### Semantic Text Variants

```rust
// Display text (largest, for hero content)
commands.spawn(
    Text::display("Welcome to Our App").build()
);

// Title text (for section titles)
commands.spawn(
    Text::title("Getting Started").build()
);

// Body text (default, for main content)
commands.spawn(
    Text::body("This is the main content of your application.").build()
);

// Label text (for form labels and UI labels)
commands.spawn(
    Text::label("Username").build()
);

// Caption text (smallest, for supplementary info)
commands.spawn(
    Text::caption("Optional field").build()
);
```

### Font Families

```rust
// Sans-serif font (default)
commands.spawn(
    Text::new("Clean modern text")
        .family(FontFamily::Sans)
        .build()
);

// Serif font (elegant, traditional)
commands.spawn(
    Text::new("Elegant serif text")
        .family(FontFamily::Serif)
        .build()
);

// Monospace font (for code)
commands.spawn(
    Text::new("fn main() { println!(\"Hello!\"); }")
        .family(FontFamily::Mono)
        .build()
);
```

### Font Weights and Styles

```rust
// All font weights
commands.spawn(Text::new("Light text").weight(TextWeight::Light).build());
commands.spawn(Text::new("Regular text").weight(TextWeight::Regular).build());
commands.spawn(Text::new("Medium text").weight(TextWeight::Medium).build());
commands.spawn(Text::new("Bold text").weight(TextWeight::Bold).build());

// Italic variations
commands.spawn(Text::new("Light italic").weight(TextWeight::Light).italic().build());
commands.spawn(Text::new("Bold italic").weight(TextWeight::Bold).italic().build());
```

### Text Colors

```rust
// Theme colors
commands.spawn(Text::new("Default color").color(TextColor::Default).build());
commands.spawn(Text::new("Muted color").color(TextColor::Muted).build());
commands.spawn(Text::new("Accent color").color(TextColor::Accent).build());

// Status colors
commands.spawn(Text::new("Error message").color(TextColor::Error).build());
commands.spawn(Text::new("Success message").color(TextColor::Success).build());
commands.spawn(Text::new("Warning message").color(TextColor::Warning).build());

// Custom color
commands.spawn(
    Text::new("Custom purple text")
        .color(TextColor::Custom(Color::srgb(0.7, 0.3, 0.9)))
        .build()
);
```

### Text Alignment

```rust
// Left aligned (default)
commands.spawn(Text::new("Left aligned text").build());

// Center aligned
commands.spawn(Text::new("Center aligned text").center().build());

// Right aligned
commands.spawn(Text::new("Right aligned text").right().build());
```

## Heading Component Examples

### Basic Heading Usage

```rust
use your_ui_crate::components::heading::{Heading, HeadingLevel, HeadingExt};

// All heading levels
commands.spawn(Heading::h1("Main Page Title").build());       // Largest
commands.spawn(Heading::h2("Section Title").build());        // Large
commands.spawn(Heading::h3("Subsection Title").build());     // Medium-Large
commands.spawn(Heading::h4("Minor Section").build());        // Medium
commands.spawn(Heading::h5("Small Heading").build());        // Small
commands.spawn(Heading::h6("Smallest Heading").build());     // Smallest
```

### Custom Styled Headings

```rust
// Heading with custom color
commands.spawn(
    Heading::h2("Special Section")
        .color(TextColor::Accent)
        .build()
);

// Heading with italic style
commands.spawn(
    Heading::h3("Stylized Heading")
        .italic()
        .build()
);

// Heading with different font family
commands.spawn(
    Heading::h1("Elegant Title")
        .family(FontFamily::Serif)
        .build()
);
```

### Using Extension Trait

```rust
// Convert regular text to heading
commands.spawn(
    Text::new("This becomes a heading")
        .as_heading(HeadingLevel::H2)
        .color(TextColor::Accent)
        .build()
);
```

## Complete Example: Article Layout

```rust
fn setup_article(mut commands: Commands) {
    commands.spawn(ui_root("main_ui"));
    
    commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(40.0)),
            row_gap: Val::Px(24.0),
            ..default()
        }
    ).with_children(|parent| {
        // Article title
        parent.spawn(
            Heading::h1("The Future of User Interfaces")
                .build()
        );

        // Article subtitle
        parent.spawn(
            Text::new("Exploring new paradigms in UI design")
                .size(TextSize::Lg)
                .color(TextColor::Muted)
                .italic()
                .build()
        );

        // Author info
        parent.spawn(
            Text::caption("By Jane Doe • March 15, 2024")
                .build()
        );

        // Section heading
        parent.spawn(
            Heading::h2("Introduction")
                .build()
        );

        // Body paragraph
        parent.spawn(
            Text::body("User interface design has evolved significantly over the past decade. Modern applications require interfaces that are not only functional but also beautiful and accessible.")
                .build()
        );

        // Another section
        parent.spawn(
            Heading::h2("Key Principles")
                .build()
        );

        // Subsection
        parent.spawn(
            Heading::h3("Accessibility First")
                .build()
        );

        // More body text
        parent.spawn(
            Text::body("Every interface should be usable by everyone, regardless of their abilities or the devices they use.")
                .build()
        );

        // Code example
        parent.spawn(
            Text::new("Text::new(\"Hello\").size(TextSize::Lg).build()")
                .family(FontFamily::Mono)
                .color(TextColor::Accent)
                .build()
        );

        // Conclusion
        parent.spawn(
            Heading::h2("Conclusion")
                .build()
        );

        parent.spawn(
            Text::body("The typography system provides a solid foundation for building consistent and beautiful user interfaces.")
                .build()
        );
    });
}
```

## Integration with Buttons

Buttons automatically use the typography system and adjust text size and weight based on button properties:

```rust
// Small button - uses TextSize::Sm
commands.spawn(
    Button::new("small_btn")
        .small()
        .text("Small Button")
        .build()
);

// Default button - uses TextSize::Base
commands.spawn(
    Button::new("default_btn")
        .text("Default Button")
        .build()
);

// Large button - uses TextSize::Lg
commands.spawn(
    Button::new("large_btn")
        .large()
        .text("Large Button")
        .build()
);
```

## Font Size Hierarchy

The typography system uses a consistent size scale:

- **Xs**: 12px - Caption text, footnotes
- **Sm**: 14px - Small labels, secondary text  
- **Base**: 16px - Body text, default size
- **Lg**: 18px - Large body text, small headings
- **Xl**: 20px - Medium headings
- **X2l**: 24px - Large headings
- **X3l**: 30px - Section titles
- **X4l**: 36px - Page titles
- **X5l**: 48px - Hero text, display headings
- **X6l** through **X9l**: 60px - 128px - Extra large display text

## Best Practices

1. **Use semantic variants** when possible (`Text::body()`, `Text::label()`) rather than manual sizing
2. **Use headings hierarchically** - don't skip levels (H1 → H2 → H3, not H1 → H3)
3. **Leverage theme colors** instead of custom colors for consistency
4. **Use italic sparingly** for emphasis, not decoration
5. **Consider accessibility** - ensure sufficient color contrast and readable font sizes

## Theme Integration

All typography components automatically integrate with your theme system:
- Font families are loaded from your `TypographyAssets`
- Colors use your theme's color palette
- Sizes scale with your theme's base font size and scaling factor
- The `update_text_fonts` system automatically applies the correct fonts when assets are loaded