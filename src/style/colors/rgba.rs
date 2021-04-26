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
        if pre_rgba.3 == 0 {
            Rgba(0, 0, 0, 0)
        } else {
            let ratio = u8::MAX as f64 / pre_rgba.3 as f64;

            Rgba(
                (pre_rgba.0 as f64 * ratio).round() as _,
                (pre_rgba.1 as f64 * ratio).round() as _,
                (pre_rgba.2 as f64 * ratio).round() as _,
                pre_rgba.3,
            )
        }
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

impl Over<Rgb> for Rgba {
    type Output = Rgb;

    fn over(self, bottom: Rgb) -> Rgb {
        PreRgba::from(self).over(bottom)
    }
}
