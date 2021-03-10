use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Color for Rgba {
    color!(self
        red       { self.0 }
        green     { self.1 }
        blue      { self.2 }
        pre_red   { self.0 * self.3 / u8::MAX }
        pre_green { self.1 * self.3 / u8::MAX }
        pre_blue  { self.2 * self.3 / u8::MAX }
        alpha     { self.3 }
    );
}

impl Rgba {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self(f(self.0), f(self.1), f(self.2), self.3)
    }
}

impl From<Rgb> for Rgba {
    fn from(Rgb(red, green, blue): Rgb) -> Self {
        Self(red, green, blue, u8::MAX)
    }
}

impl From<PreRgba> for Rgba {
    fn from(PreRgba(red, green, blue, alpha): PreRgba) -> Self {
        if alpha == 0 {
            Self::default()
        } else {
            Self(red, green, blue, alpha).map(|v| v * u8::MAX / alpha)
        }
    }
}
