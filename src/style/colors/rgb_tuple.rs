use super::*;

/// Alias of `(u8, u8, u8)`.
pub type RgbTuple = (u8, u8, u8);

impl From<Rgb> for RgbTuple {
    fn from(Rgb { red, green, blue }: Rgb) -> Self {
        (red, green, blue)
    }
}

impl From<Rgba> for RgbTuple {
    fn from(rgba: Rgba) -> Self {
        Rgb::from(rgba).into()
    }
}

impl From<PreRgba> for RgbTuple {
    fn from(pre_rgba: PreRgba) -> Self {
        Rgba::from(pre_rgba).into()
    }
}
