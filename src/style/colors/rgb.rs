use super::*;

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

color!(Rgb, self
    red       { self.0 }
    green     { self.1 }
    blue      { self.2 }
    pre_red   { self.0 }
    pre_green { self.1 }
    pre_blue  { self.2 }
    alpha     { u8::MAX }
);

impl Rgb {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

#[doc(hidden)]
impl From<Rgba> for Rgb {
    fn from(Rgba(red, green, blue, ..): Rgba) -> Rgb {
        Self(red, green, blue)
    }
}

#[doc(hidden)]
impl From<PreRgba> for Rgb {
    fn from(pre_rgba: PreRgba) -> Rgb {
        Rgba::from(pre_rgba).into()
    }
}
