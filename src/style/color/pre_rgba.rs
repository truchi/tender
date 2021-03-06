use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, Default, Hash, Debug)]
pub struct PreRgba(pub(super) u8, pub(super) u8, pub(super) u8, pub(super) u8);

impl PreRgba {
    pub fn drop_alpha(self) -> Rgb {
        debug_assert!(self.is_opaque());
        Rgb(self.0, self.1, self.2)
    }
}

impl Color for PreRgba {
    fn get_alpha(self) -> u8 {
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
        let alpha = rgba.get_alpha_f64();

        PreRgba(
            (rgba.0 as f64 * alpha).round() as _,
            (rgba.1 as f64 * alpha).round() as _,
            (rgba.2 as f64 * alpha).round() as _,
            rgba.3,
        )
    }
}

impl PartialEq<Rgb> for PreRgba {
    fn eq(&self, rgb: &Rgb) -> bool {
        rgb == self
    }
}

impl PartialEq<Rgba> for PreRgba {
    fn eq(&self, rgba: &Rgba) -> bool {
        rgba == self
    }
}

impl PartialEq<PreRgba> for PreRgba {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        self.3 == pre_rgba.3 && self.0 == pre_rgba.0 && self.1 == pre_rgba.1 && self.2 == pre_rgba.2
    }
}

impl Over<Rgb> for PreRgba {
    type Output = Rgb;

    fn over(self, bottom: Rgb) -> Rgb {
        let contr_alpha = self.get_contr_alpha_f64();

        Rgb(
            self.0 + (bottom.0 as f64 * contr_alpha).round() as u8,
            self.1 + (bottom.1 as f64 * contr_alpha).round() as u8,
            self.2 + (bottom.2 as f64 * contr_alpha).round() as u8,
        )
    }
}

impl Over<Rgba> for PreRgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgba) -> PreRgba {
        self.over(PreRgba::from(bottom))
    }
}

impl Over<PreRgba> for PreRgba {
    type Output = PreRgba;

    fn over(self, bottom: PreRgba) -> PreRgba {
        let contr_alpha = self.get_contr_alpha_f64();

        PreRgba(
            self.0 + (bottom.0 as f64 * contr_alpha).round() as u8,
            self.1 + (bottom.1 as f64 * contr_alpha).round() as u8,
            self.2 + (bottom.2 as f64 * contr_alpha).round() as u8,
            self.3 + (bottom.3 as f64 * contr_alpha).round() as u8,
        )
    }
}
