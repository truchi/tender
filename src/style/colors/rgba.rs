use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl WithAlpha for Rgba {
    fn alpha(self) -> u8 {
        self.3
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Rgba {
        Rgba(rgb.0, rgb.1, rgb.2, u8::MAX)
    }
}

impl From<PreRgba> for Rgba {
    fn from(pre_rgba: PreRgba) -> Rgba {
        let Rgb(red, green, blue) = pre_rgba.into();

        Rgba(red, green, blue, pre_rgba.alpha())
    }
}

impl PartialEq<Rgb> for Rgba {
    fn eq(&self, rgb: &Rgb) -> bool {
        *self == Rgba::from(*rgb)
    }
}

impl PartialEq<PreRgba> for Rgba {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        PreRgba::from(*self) == *pre_rgba
    }
}

impl<T, U> Over<T, U> for Rgba
where
    PreRgba: Over<T, U>,
{
    fn over(self, bottom: T) -> U {
        PreRgba::from(self).over(bottom)
    }
}
