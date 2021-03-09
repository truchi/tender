use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

color!(PreRgba, self
    red       { self.0 * u8::MAX / self.3 }
    green     { self.1 * u8::MAX / self.3 }
    blue      { self.2 * u8::MAX / self.3 }
    pre_red   { self.0 }
    pre_green { self.1 }
    pre_blue  { self.2 }
    alpha     { self.3 }
);

impl PreRgba {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self(f(self.0), f(self.1), f(self.2), self.3)
    }
}

#[doc(hidden)]
impl From<Rgb> for PreRgba {
    fn from(rgb: Rgb) -> Self {
        Rgba::from(rgb).into()
    }
}

#[doc(hidden)]
impl From<Rgba> for PreRgba {
    fn from(Rgba(red, green, blue, alpha): Rgba) -> Self {
        Self(red, green, blue, alpha).map(|v| v * alpha / u8::MAX)
    }
}
