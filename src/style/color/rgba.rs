use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, Default, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Color for Rgba {
    fn get_alpha(self) -> u8 {
        self.3
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Rgba {
        Rgba(rgb.0, rgb.1, rgb.2, u8::MAX)
    }
}

impl TryFrom<PreRgba> for Rgba {
    type Error = ();

    fn try_from(pre_rgba: PreRgba) -> Result<Rgba, ()> {
        if pre_rgba.is_visible() {
            let inv_alpha = pre_rgba.get_inv_alpha_f64();

            Ok(Rgba(
                (pre_rgba.0 as f64 * inv_alpha).round() as _,
                (pre_rgba.1 as f64 * inv_alpha).round() as _,
                (pre_rgba.2 as f64 * inv_alpha).round() as _,
                pre_rgba.3,
            ))
        } else {
            Err(())
        }
    }
}

impl PartialEq<Rgb> for Rgba {
    fn eq(&self, rgb: &Rgb) -> bool {
        rgb == self
    }
}

impl PartialEq<Rgba> for Rgba {
    fn eq(&self, rgba: &Rgba) -> bool {
        self.3 == rgba.3 && self.0 == rgba.0 && self.1 == rgba.1 && self.2 == rgba.2
    }
}

impl PartialEq<PreRgba> for Rgba {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        Rgba::try_from(*pre_rgba)
            .map(|rgba| *self == rgba)
            .unwrap_or(false)
    }
}

impl Over<Rgb> for Rgba {
    type Output = Rgb;

    fn over(self, bottom: Rgb) -> Rgb {
        PreRgba::from(self).over(bottom)
    }
}

impl Over<Rgba> for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgba) -> PreRgba {
        PreRgba::from(self).over(bottom)
    }
}

impl Over<PreRgba> for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: PreRgba) -> PreRgba {
        PreRgba::from(self).over(bottom)
    }
}
