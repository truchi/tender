//! Colors ([`Rgb`], [`Rgba`], [`PreRgba`]).

mod ground;
mod pre_rgba;
mod rgb;
mod rgba;

pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;

use super::*;
use std::fmt::{self, Display, Formatter};

/// A wrapper type for colors.
#[derive(Copy, Clone, Eq, Default, Hash, Debug)]
pub struct Color<T>(pub T);

impl<T> From<T> for Color<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T: PartialEq<U>, U> PartialEq<Color<U>> for Color<T> {
    fn eq(&self, other: &Color<U>) -> bool {
        &self.0 == &other.0
    }
}

impl<T: WithAlpha> WithAlpha for Color<T> {
    fn alpha(self) -> u8 {
        self.0.alpha()
    }
}

impl<C: Over<T, U>, T, U> Over<Color<T>, Color<U>> for Color<C> {
    fn over(self, bottom: Color<T>) -> Color<U> {
        Color(self.0.over(bottom.0))
    }
}

/// A trait for [`Rgb`], [`Rgba`], [`PreRgba`].
pub trait WithAlpha: Copy + Default
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

    /// Wraps `self` within a [`Color`].
    fn wrap(self) -> Color<Self> {
        self.into()
    }
}
