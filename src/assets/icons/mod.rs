pub mod macros;

// Interface Icons (UI Icons)
pub mod interface;

// Controller Icons  
pub mod controllers;

// Loading System
pub mod loading;

// Re-export nur IconSize (Makros sind global verfügbar)
pub use macros::IconSize;

// Re-export aller Icon-Kategorien
pub use interface::*;
pub use controllers::*;

// Re-export der Loading-Funktion
pub use loading::*;

// IconSize wird durch das Makro generiert und ist verfügbar