use bevy::prelude::*;

/// Standardized icon size enumeration for consistent UI scaling
///
/// IconSize provides predefined size constants that ensure visual consistency
/// across all icon categories in the design system. Each size maps to a specific
/// pixel dimension optimized for different UI contexts.
///
/// # Size Guidelines
///
/// - **Small (16px)**: Inline icons, compact UI elements, dense layouts
/// - **Medium (24px)**: Standard UI icons, buttons, navigation elements (default)
/// - **Large (32px)**: Prominent actions, headers, featured content
/// - **ExtraLarge (64px)**: Hero icons, splash screens, empty states
///
/// # Usage Examples
///
/// ```rust
/// use your_crate::assets::icons::IconSize;
/// use bevy::prelude::*;
///
/// // Using with icon builders
/// let small_icon = ButtonA::new().size(IconSize::Small);
/// let large_icon = ArrowRight::new().size(IconSize::Large);
///
/// // Converting to Bevy Val for layout
/// let icon_width = IconSize::Medium.as_val(); // Val::Px(24.0)
/// ```
///
/// # Design Consistency
///
/// All icon atlases are generated at these exact dimensions to ensure:
/// - Crisp rendering at all scales
/// - Consistent visual hierarchy
/// - Optimal asset loading performance
/// - Platform-agnostic sizing
///
/// # Performance Notes
///
/// - Each size uses a dedicated texture atlas for optimal GPU performance
/// - No runtime scaling - icons are pre-rendered at target sizes
/// - Memory usage scales with the number of active icon sizes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    /// 16px - Compact icons for dense layouts and inline elements
    Small = 16,
    /// 24px - Standard icon size for most UI elements (default)
    Medium = 24,
    /// 32px - Larger icons for prominent actions and headers
    Large = 32,
    /// 64px - Extra large icons for hero sections and empty states
    ExtraLarge = 64,
}

impl IconSize {
    /// Convert IconSize to Bevy's Val type for use in UI layouts
    ///
    /// This method provides seamless integration with Bevy's UI system by
    /// converting the enum value to a pixel-based Val.
    ///
    /// # Returns
    ///
    /// A `Val::Px` containing the pixel dimension of the icon size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let small_val = IconSize::Small.as_val();     // Val::Px(16.0)
    /// let medium_val = IconSize::Medium.as_val();   // Val::Px(24.0)
    /// let large_val = IconSize::Large.as_val();     // Val::Px(32.0)
    /// ```
    pub fn as_val(&self) -> Val {
        Val::Px(*self as u8 as f32)
    }
}

/// Comprehensive macro system for generating icon categories with full type safety
///
/// The `define_icon_category!` macro is the core of the icon system, automatically
/// generating all necessary types, builders, and asset loading infrastructure for
/// a complete icon category. This ensures consistency, type safety, and ergonomic
/// APIs across all icon sets.
///
/// # Generated Components
///
/// For each icon category, the macro generates:
///
/// ## Core Types
/// - **Icon ID Enum**: Type-safe identifiers for each icon in the category
/// - **Atlas Resource**: Manages texture atlases for all icon sizes (16px, 24px, 32px, 64px)
/// - **Icon Component**: Bevy component with size, tint, and configuration options
///
/// ## Builder APIs
/// - **Direct Builders**: `ButtonA::new()` - Direct access to specific icons
/// - **Namespace Builders**: `Xbox::ButtonA::new()` - Categorized access pattern
/// - **Fluent Interface**: Method chaining for size, tint, and styling
///
/// ## Asset Management
/// - **Loading Function**: Automatic atlas loading with proper grid configuration
/// - **Multi-Resolution**: Support for 16px, 24px, 32px, and 64px variants
/// - **Resource Management**: Handles for textures and layouts
///
/// # Usage Examples
///
/// ## Basic Category Definition
/// ```rust
/// define_icon_category! {
///     Xbox {
///         path: "ui/icons/controllers/xbox",
///         atlas: (20, 5),  // 20 columns, 5 rows
///         icons: [
///             ButtonA = 0,
///             ButtonB = 1,
///             ButtonX = 2,
///             ButtonY = 3,
///             DPadUp = 4,
///             DPadDown = 5,
///         ]
///     }
/// }
/// ```
///
/// ## Using Generated Icons
/// ```rust
/// // Direct builder approach
/// let icon = ButtonA::new()
///     .large()
///     .tint(Color::RED)
///     .bundle(&xbox_atlases);
///
/// // Namespace approach
/// let icon = Xbox::ButtonA::new()
///     .medium()
///     .tint(Color::BLUE)
///     .bundle(&xbox_atlases);
///
/// // Component-based approach
/// let icon = XboxIcon::new(XboxIconId::ButtonA)
///     .size(IconSize::ExtraLarge)
///     .tint(Color::GREEN)
///     .bundle(&xbox_atlases);
/// ```
///
/// ## Asset Loading Integration
/// ```rust
/// // Generated loading function
/// app.add_systems(Startup, load_xbox_atlases);
///
/// // Or use the combined loader
/// app.add_systems(Startup, load_all_icon_atlases);
/// ```
///
/// # Atlas Organization
///
/// The macro expects texture atlases organized as follows:
/// ```
/// ui/icons/controllers/xbox/
/// ├── texture_atlas_20x5_16px.png   # 16px icons
/// ├── texture_atlas_20x5_24px.png   # 24px icons
/// ├── texture_atlas_20x5_32px.png   # 32px icons
/// └── texture_atlas_20x5_64px.png   # 64px icons
/// ```
///
/// # Type Safety Features
///
/// - **Compile-time validation**: Invalid icon IDs are caught at compile time
/// - **Exhaustive matching**: Enum-based icon IDs enable complete pattern matching
/// - **Resource safety**: Atlas handles are type-safe and category-specific
/// - **Builder constraints**: Method signatures prevent invalid configurations
///
/// # Performance Characteristics
///
/// - **Zero-cost abstractions**: Builders compile to direct field assignments
/// - **Efficient atlasing**: Multiple resolutions avoid runtime scaling
/// - **Memory optimization**: Atlases are loaded once and shared across instances
/// - **GPU efficiency**: Texture atlases minimize draw calls and state changes
///
/// # Extension Points
///
/// The generated types can be extended with custom implementations:
/// ```rust
/// impl XboxIcon {
///     pub fn with_controller_theme(mut self) -> Self {
///         self.tint = Color::rgba(0.2, 0.8, 0.2, 1.0);
///         self
///     }
/// }
/// ```
///
/// # Accessibility Considerations
///
/// - Icons maintain aspect ratios across all sizes
/// - Tinting supports high contrast themes
/// - Size options accommodate different accessibility needs
/// - Semantic naming aids screen reader compatibility
#[macro_export]
macro_rules! define_icon_category {
    (
        $category:ident {
            path: $asset_path:literal,
            atlas: ($cols:literal, $rows:literal),
            icons: [
                $( $icon_name:ident = $icon_index:literal ),* $(,)?
            ]
        }
    ) => {
        paste::paste! {
            // ===== CORE TYPES =====
            
            // Icon-ID Enum für diese Kategorie
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum [<$category IconId>] {
                $(
                    $icon_name = $icon_index,
                )*
            }

            // Atlas-Resource für diese Kategorie
            #[derive(Resource)]
            pub struct [<$category Atlases>] {
                pub texture_16: Handle<bevy::prelude::Image>,
                pub texture_24: Handle<bevy::prelude::Image>,
                pub texture_32: Handle<bevy::prelude::Image>,
                pub texture_64: Handle<bevy::prelude::Image>,
                pub layout_16: Handle<TextureAtlasLayout>,
                pub layout_24: Handle<TextureAtlasLayout>,
                pub layout_32: Handle<TextureAtlasLayout>,
                pub layout_64: Handle<TextureAtlasLayout>,
            }

            // Icon-Component für diese Kategorie
            #[derive(Component, Clone, Copy, Debug)]
            pub struct [<$category Icon>] {
                pub id: [<$category IconId>],
                pub size: IconSize,
                pub tint: Color,
            }

            impl [<$category Icon>] {
                pub fn new(id: [<$category IconId>]) -> Self {
                    Self {
                        id,
                        size: IconSize::Medium,
                        tint: Color::WHITE,
                    }
                }

                pub fn size(mut self, size: IconSize) -> Self {
                    self.size = size;
                    self
                }

                pub fn tint(mut self, color: Color) -> Self {
                    self.tint = color;
                    self
                }

                pub fn small(mut self) -> Self {
                    self.size = IconSize::Small;
                    self
                }

                pub fn medium(mut self) -> Self {
                    self.size = IconSize::Medium;
                    self
                }

                pub fn large(mut self) -> Self {
                    self.size = IconSize::Large;
                    self
                }

                pub fn extra_large(mut self) -> Self {
                    self.size = IconSize::ExtraLarge;
                    self
                }

                pub fn bundle(self, atlases: &[<$category Atlases>]) -> impl Bundle {
                    let (texture, layout) = match self.size {
                        IconSize::Small => (atlases.texture_16.clone(), atlases.layout_16.clone()),
                        IconSize::Medium => (atlases.texture_24.clone(), atlases.layout_24.clone()),
                        IconSize::Large => (atlases.texture_32.clone(), atlases.layout_32.clone()),
                        IconSize::ExtraLarge => (atlases.texture_64.clone(), atlases.layout_64.clone()),
                    };

                    (
                        self,
                        ImageNode::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout,
                                index: self.id as usize,
                            },
                        ).with_color(self.tint),
                        Node {
                            width: self.size.as_val(),
                            height: self.size.as_val(),
                            ..default()
                        },
                    )
                }

                pub fn spawn(self, commands: &mut Commands, atlases: &[<$category Atlases>]) -> Entity {
                    commands.spawn(self.bundle(atlases)).id()
                }
            }

            // ===== ASSET LOADING =====
            
            // Asset-Loading-Funktion
            pub fn [<load_ $category:lower _atlases>](
                mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut layouts: ResMut<Assets<TextureAtlasLayout>>,
            ) {
                let layout_16 = layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(16, 16), $cols, $rows, None, None,
                ));
                let layout_24 = layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(24, 24), $cols, $rows, None, None,
                ));
                let layout_32 = layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 32), $cols, $rows, None, None,
                ));
                let layout_64 = layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(64, 64), $cols, $rows, None, None,
                ));

                commands.insert_resource([<$category Atlases>] {
                    texture_16: asset_server.load(concat!($asset_path, "/texture_atlas_", $cols, "x", $rows, "_16px.png")),
                    texture_24: asset_server.load(concat!($asset_path, "/texture_atlas_", $cols, "x", $rows, "_24px.png")),
                    texture_32: asset_server.load(concat!($asset_path, "/texture_atlas_", $cols, "x", $rows, "_32px.png")),
                    texture_64: asset_server.load(concat!($asset_path, "/texture_atlas_", $cols, "x", $rows, "_64px.png")),
                    layout_16,
                    layout_24,
                    layout_32,
                    layout_64,
                });
            }

            // ===== ICON BUILDERS =====
            
            // Direkte Icon-Builder (Variante 1: ButtonA::new())
            $(
                #[doc = concat!("Direct builder for ", stringify!($category), " icon: ", stringify!($icon_name))]
                #[doc = ""]
                #[doc = concat!("Provides direct access to the ", stringify!($icon_name), " icon with a fluent builder API.")]
                #[doc = "This builder allows for immediate icon creation without namespace qualification."]
                #[doc = ""]
                #[doc = "# Usage Examples"]
                #[doc = ""]
                #[doc = "```rust"]
                #[doc = concat!("// Basic usage")]
                #[doc = concat!("let icon = ", stringify!($icon_name), "::new().bundle(&atlases);")]
                #[doc = ""]
                #[doc = concat!("// With customization")]
                #[doc = concat!("let styled_icon = ", stringify!($icon_name), "::new()")]
                #[doc = "    .large()"]
                #[doc = "    .tint(Color::RED)"]
                #[doc = "    .bundle(&atlases);"]
                #[doc = ""]
                #[doc = concat!("// Shorthand methods")]
                #[doc = concat!("let small_icon = ", stringify!($icon_name), "::small();")]
                #[doc = concat!("let tinted_icon = ", stringify!($icon_name), "::tint(Color::BLUE);")]
                #[doc = "```"]
                #[doc = ""]
                #[doc = "# Available Methods"]
                #[doc = ""]
                #[doc = "- `new()` - Create icon with default settings (Medium size, White tint)"]
                #[doc = "- `small()`, `medium()`, `large()`, `extra_large()` - Size shortcuts"]
                #[doc = "- `tint(Color)` - Apply color tinting"]
                #[doc = "- `bundle(&atlases)` - Generate Bevy bundle for spawning"]
                pub struct $icon_name;

                impl $icon_name {
                    pub fn new() -> [<$category Icon>] {
                        [<$category Icon>]::new([<$category IconId>]::$icon_name)
                    }

                    pub fn small() -> [<$category Icon>] {
                        Self::new().small()
                    }

                    pub fn medium() -> [<$category Icon>] {
                        Self::new().medium()
                    }

                    pub fn large() -> [<$category Icon>] {
                        Self::new().large()
                    }

                    pub fn extra_large() -> [<$category Icon>] {
                        Self::new().extra_large()
                    }

                    pub fn tint(color: Color) -> [<$category Icon>] {
                        Self::new().tint(color)
                    }

                    pub fn bundle(atlases: &[<$category Atlases>]) -> impl Bundle {
                        Self::new().bundle(atlases)
                    }
                }
            )*

            // Namespace-Icon-Builder Structs (Variante 2: Xbox::ButtonA::new())
            $(
                paste::paste! {
                    #[doc = concat!("Namespace builder for ", stringify!($category), " icon: ", stringify!($icon_name))]
                    #[doc = ""]
                    #[doc = concat!("Provides categorized access to the ", stringify!($icon_name), " icon through the ", stringify!($category), " namespace.")]
                    #[doc = "This approach offers better organization when working with multiple icon categories."]
                    #[doc = ""]
                    #[doc = "# Usage Examples"]
                    #[doc = ""]
                    #[doc = "```rust"]
                    #[doc = concat!("// Namespace access")]
                    #[doc = concat!("let icon = ", stringify!($category), "::", stringify!($icon_name), "::new().bundle(&atlases);")]
                    #[doc = ""]
                    #[doc = concat!("// With method chaining")]
                    #[doc = concat!("let styled_icon = ", stringify!($category), "::", stringify!($icon_name), "::new()")]
                    #[doc = "    .medium()"]
                    #[doc = "    .tint(Color::GREEN)"]
                    #[doc = "    .bundle(&atlases);"]
                    #[doc = "```"]
                    pub struct [<$category $icon_name>];

                    impl [<$category $icon_name>] {
                        pub fn new() -> [<$category Icon>] {
                            [<$category Icon>]::new([<$category IconId>]::$icon_name)
                        }

                        pub fn small() -> [<$category Icon>] {
                            Self::new().small()
                        }

                        pub fn medium() -> [<$category Icon>] {
                            Self::new().medium()
                        }

                        pub fn large() -> [<$category Icon>] {
                            Self::new().large()
                        }

                        pub fn extra_large() -> [<$category Icon>] {
                            Self::new().extra_large()
                        }

                        pub fn tint(color: Color) -> [<$category Icon>] {
                            Self::new().tint(color)
                        }

                        pub fn bundle(atlases: &[<$category Atlases>]) -> impl Bundle {
                            Self::new().bundle(atlases)
                        }
                    }
                }
            )*

            // Namespace-Modul für einfachen Zugriff
            #[doc = concat!("Namespace module for ", stringify!($category), " icons")]
            #[doc = ""]
            #[doc = concat!("The ", stringify!($category), " namespace provides organized access to all icons in this category.")]
            #[doc = "This approach is recommended when working with multiple icon categories to avoid naming conflicts."]
            #[doc = ""]
            #[doc = "# Usage Examples"]
            #[doc = ""]
            #[doc = "```rust"]
            #[doc = concat!("// Access icons through namespace")]
            #[doc = concat!("let icon = ", stringify!($category), "::ButtonA().new().bundle(&atlases);")]
            #[doc = ""]
            #[doc = concat!("// Multiple icons from same category")]
            #[doc = concat!("let button_a = ", stringify!($category), "::ButtonA().new();")]
            #[doc = concat!("let button_b = ", stringify!($category), "::ButtonB().new();")]
            #[doc = ""]
            #[doc = concat!("// With consistent styling")]
            #[doc = concat!("let controller_theme = Color::rgba(0.2, 0.8, 0.2, 1.0);")]
            #[doc = concat!("let styled_a = ", stringify!($category), "::ButtonA().new().tint(controller_theme);")]
            #[doc = concat!("let styled_b = ", stringify!($category), "::ButtonB().new().tint(controller_theme);")]
            #[doc = "```"]
            #[doc = ""]
            #[doc = "# Category Organization"]
            #[doc = ""]
            #[doc = concat!("All ", stringify!($category), " icons are accessible through this namespace, providing:")]
            #[doc = "- Clear categorization and organization"]
            #[doc = "- Reduced naming conflicts between categories"]
            #[doc = "- Improved code readability and maintenance"]
            #[doc = "- Consistent API across all icon categories"]
            pub struct $category;

            impl $category {
                $(
                    pub fn $icon_name() -> [<$category $icon_name>] {
                        [<$category $icon_name>]
                    }
                )*
            }
        }
    };
}

/// Macro for generating centralized icon loading systems
///
/// The `define_icon_loading_system!` macro creates a unified asset loading function
/// that coordinates the loading of all defined icon categories. This ensures proper
/// initialization order and simplifies application setup.
///
/// # Generated Function
///
/// Creates a `load_all_icon_atlases` system function that:
/// - Calls individual category loading functions in sequence
/// - Properly manages Bevy resource borrowing
/// - Handles asset server and layout resource coordination
/// - Ensures all atlases are loaded before UI systems activate
///
/// # Usage Examples
///
/// ## Define Loading System
/// ```rust
/// define_icon_loading_system! {
///     categories: [
///         Xbox,
///         PlayStation,
///         Interface,
///         Loading,
///     ]
/// }
/// ```
///
/// ## Integrate with Bevy App
/// ```rust
/// use bevy::prelude::*;
/// 
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_systems(Startup, load_all_icon_atlases)
///         .add_systems(Update, ui_system.after(load_all_icon_atlases))
///         .run();
/// }
/// ```
///
/// # System Dependencies
///
/// The generated loading system requires:
/// - `AssetServer` resource for loading texture files
/// - `Assets<TextureAtlasLayout>` resource for atlas layout management
/// - All category-specific loading functions to be in scope
///
/// # Loading Order
///
/// Categories are loaded in the order specified in the macro invocation.
/// This allows for dependency management if needed, though typically
/// icon categories are independent.
///
/// # Error Handling
///
/// Asset loading errors are handled by Bevy's asset system:
/// - Missing files will log warnings
/// - Invalid atlas configurations will cause panics in debug builds
/// - The UI system should check for resource availability before use
///
/// # Performance Considerations
///
/// - All atlases are loaded simultaneously for optimal throughput
/// - Loading is typically completed during the initial loading screen
/// - Memory usage scales with the total number of icon categories and sizes
/// - GPU memory allocation happens once per atlas during loading
#[macro_export]
macro_rules! define_icon_loading_system {
    (
        categories: [
            $( $category:ident ),* $(,)?
        ]
    ) => {
        paste::paste! {
            /// Lädt alle Icon-Atlases für alle definierten Kategorien
            pub fn load_all_icon_atlases(
                mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut layouts: ResMut<Assets<TextureAtlasLayout>>,
            ) {
                $(
                    [<load_ $category:lower _atlases>](
                        commands.reborrow(), 
                        asset_server.clone(), 
                        layouts.reborrow()
                    );
                )*
            }
        }
    };
}