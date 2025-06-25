use super::generics::*;

/// Ein Makro, um spezifische Icon-Konstruktor-Structs zu generieren.
///
/// ## Verwendung
///
/// ```
/// define_specific_icons! {
///     // StructName => IconId Enum-Variante
///     ChevronDown => ChevronDown,
///     Check => Check
/// }
/// ```
///
/// Dies generiert:
///
/// ```
/// pub struct ChevronDown;
/// impl ChevronDown {
///     pub fn new() -> Icon { Icon::new(IconId::ChevronDown) }
///     // ... plus Helfer wie .small(), .large() etc.
/// }
/// ```
#[macro_export]
macro_rules! define_specific_icons {
    ( $( $struct_name:ident => $icon_id:ident ),* $(,)? ) => {
        $(
            #[doc = concat!("Ein spezifischer Icon-Builder für `", stringify!($struct_name), "`.")]
            #[allow(non_snake_case)]
            pub struct $struct_name;

            #[allow(dead_code)]
            impl $struct_name {
                #[doc = "Erstellt eine neue `Icon`-Instanz für dieses Icon."]
                pub fn new() -> Icon {
                    Icon::new(IconId::$icon_id)
                }

                #[doc = "Erstellt eine neue kleine `Icon`-Instanz."]
                pub fn small() -> Icon {
                    Icon::new(IconId::$icon_id).size(IconSize::Small)
                }

                #[doc = "Erstellt eine neue mittlere `Icon`-Instanz."]
                pub fn medium() -> Icon {
                    Icon::new(IconId::$icon_id).size(IconSize::Medium)
                }

                #[doc = "Erstellt eine neue große `Icon`-Instanz."]
                pub fn large() -> Icon {
                    Icon::new(IconId::$icon_id).size(IconSize::Large)
                }

                #[doc = "Erstellt eine neue extra große `Icon`-Instanz."]
                pub fn extra_large() -> Icon {
                    Icon::new(IconId::$icon_id).size(IconSize::ExtraLarge)
                }
            }
        )*
    };
}
