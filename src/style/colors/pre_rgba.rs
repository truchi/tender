use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl PreRgba {
    pub fn pre_red(self) -> u8 {
        self.0
    }

    pub fn pre_green(self) -> u8 {
        self.1
    }

    pub fn pre_blue(self) -> u8 {
        self.2
    }

    pub fn alpha(self) -> u8 {
        self.3
    }

    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self(f(self.0), f(self.1), f(self.2), self.3)
    }

    /// Places `self` over `other`.
    pub fn over(self, other: Rgb) -> Rgb {
        let over = |a, b| a + b * (u8::MAX - self.3);

        Rgb(
            over(self.0, other.0),
            over(self.1, other.1),
            over(self.2, other.2),
        )
    }
}

impl From<Rgb> for PreRgba {
    fn from(rgb: Rgb) -> Self {
        Rgba::from(rgb).into()
    }
}

impl From<Rgba> for PreRgba {
    fn from(Rgba(red, green, blue, alpha): Rgba) -> Self {
        Self(red, green, blue, alpha).map(|v| v * alpha / u8::MAX)
    }
}
