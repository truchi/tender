use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Color for Rgba {
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

impl Over<Rgb> for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgb) -> PreRgba {
        PreRgba::from(self).over(bottom)
    }
}

impl Over for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgba) -> PreRgba {
        PreRgba::from(self).over(PreRgba::from(bottom))
    }
}

impl Over<PreRgba> for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: PreRgba) -> PreRgba {
        PreRgba::from(self).over(bottom)
    }
}
