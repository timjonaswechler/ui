use bevy::prelude::*;

// Gemeinsame Icon-Size Definition für alle Kategorien
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    Small = 16,
    Medium = 24,
    Large = 32,
    ExtraLarge = 64,
}

impl IconSize {
    pub fn as_val(&self) -> Val {
        Val::Px(*self as u8 as f32)
    }
}

/// Einheitliches Makro für alle Icon-Kategorien
/// 
/// Generiert für jede Icon-Kategorie:
/// - Enum für Icon-IDs
/// - Atlas-Resource-Struktur
/// - Icon-Component mit Builder-Methoden
/// - Asset-Loading-Funktion
/// - Direkte Icon-Builder (ButtonA::new())
/// - Namespace-Icon-Builder (Xbox::ButtonA::new())
///
/// Verwendung:
/// ```
/// define_icon_category! {
///     Xbox {
///         path: "ui/icons/controllers/xbox",
///         atlas: (20, 5),
///         icons: [
///             ButtonA = 0,
///             ButtonB = 1,
///             ButtonX = 2,
///             ButtonY = 3,
///         ]
///     }
/// }
/// ```
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
                #[doc = "Usage:"]
                #[doc = concat!("```rust")]
                #[doc = concat!("let icon = ", stringify!($icon_name), "::new().tint(Color::RED).bundle(&atlases);")]
                #[doc = "```"]
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
            #[doc = "Usage:"]
            #[doc = concat!("```rust")]
            #[doc = concat!("let icon = ", stringify!($category), "::ButtonA::new().tint(Color::RED).bundle(&atlases);")]
            #[doc = "```"]
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

/// Makro für die zentrale Icon-Loading-Funktion
/// 
/// Generiert eine Funktion die alle Atlas-Loading-Funktionen aufruft
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