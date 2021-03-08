use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
    pub alpha: u8,
}

impl Rgba {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self {
            red:   f(self.red),
            green: f(self.green),
            blue:  f(self.blue),
            alpha: self.alpha,
        }
    }
}

impl From<RgbTuple> for Rgba {
    fn from(rgb: RgbTuple) -> Self {
        Rgb::from(rgb).into()
    }
}

impl From<Rgb> for Rgba {
    fn from(Rgb { red, green, blue }: Rgb) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: u8::MAX,
        }
    }
}

impl From<PreRgba> for Rgba {
    fn from(
        PreRgba {
            red,
            green,
            blue,
            alpha,
        }: PreRgba,
    ) -> Self {
        if alpha == 0 {
            Self::default()
        } else {
            Self {
                red,
                green,
                blue,
                alpha,
            }
            .map(|v| v * u8::MAX / alpha)
        }
    }
}
