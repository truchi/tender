//! Colors ([`Rgb`], [`Rgba`], [`PreRgba`]).

// mod ground;
mod pre_rgba;
mod rgb;
mod rgba;

// pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;

use super::*;

/// A trait for [`Rgb`], [`Rgba`], [`PreRgba`].
pub trait Color:
    Copy
    + Default
    + From<Rgb>
    + From<Rgba>
    + From<PreRgba>
    + Into<Rgb>
    + Into<Rgba>
    + Into<PreRgba>
    + Over<Rgb>
    + Over<Rgba>
    + Over<Rgba>
{
    /// Returns the `alpha` component's value.
    fn alpha(self) -> u8;

    /// Returns `true` if `alpha == u8::MAX`.
    fn is_opaque(self) -> bool {
        self.alpha() == u8::MAX
    }

    /// Returns `true` if `alpha != u8::MAX`.
    fn is_transparent(self) -> bool {
        self.alpha() != u8::MAX
    }

    /// Returns `true` if `alpha != 0`.
    fn is_visible(self) -> bool {
        self.alpha() != 0
    }

    /// Returns `true` if `alpha == 0`.
    fn is_invisible(self) -> bool {
        self.alpha() == 0
    }
}
