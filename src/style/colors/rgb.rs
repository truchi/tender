use super::*;

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn red(self) -> u8 {
        self.0
    }

    pub fn green(self) -> u8 {
        self.1
    }

    pub fn blue(self) -> u8 {
        self.2
    }

    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

impl From<Rgba> for Rgb {
    fn from(Rgba(red, green, blue, ..): Rgba) -> Rgb {
        Self(red, green, blue)
    }
}

impl From<PreRgba> for Rgb {
    fn from(pre_rgba: PreRgba) -> Rgb {
        Rgba::from(pre_rgba).into()
    }
}
