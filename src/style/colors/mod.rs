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

/// A wrapper type for colors.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct ColorWrapper<T>(pub T);

impl<T> From<T> for ColorWrapper<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

// impl<C, T, U> Over<T, U> for ColorWrapper<C> {
// fn over(self, bottom: T) -> U {
// self.0.over(bottom)
// }
// }

impl<T: Color> Color for ColorWrapper<T> {
    fn alpha(self) -> u8 {
        self.0.alpha()
    }
}

/// A trait for [`Rgb`], [`Rgba`], [`PreRgba`].
pub trait Color: Copy + Default
// + From<Rgb>
// + From<Rgba>
// + From<PreRgba>
// + Into<Rgb>
// + Into<Rgba>
// + Into<PreRgba>
// + Over<Rgb, Rgb>
// + Over<Rgb, Rgba>
// + Over<Rgb, PreRgba>
// + Over<Rgba, Rgb>
// + Over<Rgba, Rgba>
// + Over<Rgba, PreRgba>
// + Over<PreRgba, Rgb>
// + Over<PreRgba, Rgba>
// + Over<PreRgba, PreRgba>
{
    /// Returns the `alpha` component's value.
    fn alpha(self) -> u8;

    /// Returns the `alpha` component's value as `f64` ([0.0, 1.0]).
    fn alpha_f64(self) -> f64 {
        self.alpha() as f64 / u8::MAX as f64
    }

    /// Returns `1.0 / alpha_f64`, or `None` if `alpha == 0`.
    fn inv_alpha_f64(self) -> Option<f64> {
        let alpha = self.alpha();

        if alpha == 0 {
            None
        } else {
            Some(u8::MAX as f64 / self.alpha() as f64)
        }
    }

    /// Returns `1.0 - alpha_f64`.
    fn contr_alpha_f64(self) -> f64 {
        1.0 - self.alpha_f64()
    }

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

    /// Wraps `self` within a [`ColorWrapper`].
    fn wrap(self) -> ColorWrapper<Self> {
        self.into()
    }
}
