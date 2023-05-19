//! # Art
//! 
//! A library for modeling artistic concepts.

// We re-export the enums and functions within these two modules.
// This means that these will be exported at the top level of our library.
// 'pub use' statements allow us to make our public API different from the internal structure of our program.
// Can be VERY useful for people consuming our library.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {

    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    } 
}