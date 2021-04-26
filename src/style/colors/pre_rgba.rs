use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl Color for PreRgba {
    fn alpha(self) -> u8 {
        self.3
    }
}

impl From<Rgb> for PreRgba {
    fn from(rgb: Rgb) -> PreRgba {
        PreRgba(rgb.0, rgb.1, rgb.2, u8::MAX)
    }
}

impl From<Rgba> for PreRgba {
    fn from(rgba: Rgba) -> PreRgba {
        let ratio = rgba.3 as f64 / u8::MAX as f64;

        PreRgba(
            (rgba.0 as f64 * ratio).round() as _,
            (rgba.1 as f64 * ratio).round() as _,
            (rgba.2 as f64 * ratio).round() as _,
            rgba.3,
        )
    }
}

impl Over for PreRgba {
    type Output = PreRgba;

    fn over(self, bottom: PreRgba) -> PreRgba {
        let ratio = 1.0 - (self.3 as f64 / u8::MAX as f64);

        PreRgba(
            self.0 + (bottom.0 as f64 * ratio).round() as u8,
            self.1 + (bottom.1 as f64 * ratio).round() as u8,
            self.2 + (bottom.2 as f64 * ratio).round() as u8,
            self.3 + (bottom.3 as f64 * ratio).round() as u8,
        )
    }
}

impl Over<Rgb> for PreRgba {
    type Output = Rgb;

    fn over(self, bottom: Rgb) -> Rgb {
        let ratio = 1.0 - (self.3 as f64 / u8::MAX as f64);

        Rgb(
            self.0 + (bottom.0 as f64 * ratio).round() as u8,
            self.1 + (bottom.1 as f64 * ratio).round() as u8,
            self.2 + (bottom.2 as f64 * ratio).round() as u8,
        )
    }
}

impl Over<Rgba> for PreRgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgba) -> PreRgba {
        self.over(PreRgba::from(bottom))
    }
}
