//! Colors ([`Color`]: [`Rgb`], [`Rgba`], [`PreRgba`]).

mod ground;
mod pre_rgba;
mod rgb;
mod rgba;
mod with_alpha;

pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;
pub use with_alpha::*;

use super::*;

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
