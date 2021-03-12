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

impl From<Rgb> for Rgba {
    fn from(color: Rgb) -> Self {
        Self(color.red(), color.green(), color.blue(), u8::MAX)
    }
}

impl From<PreRgba> for Rgba {
    fn from(PreRgba(red, green, blue, alpha): PreRgba) -> Self {
        if alpha == 0 {
            Self(0, 0, 0, 0)
        } else {
            Self(
                red * u8::MAX / alpha,
                green * u8::MAX / alpha,
                blue * u8::MAX / alpha,
                alpha,
            )
        }
    }
}
