use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl WithAlpha for PreRgba {
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
        let alpha = rgba.alpha_f64();

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
        *self == PreRgba::from(*rgb)
    }
}

impl PartialEq<Rgba> for PreRgba {
    fn eq(&self, rgba: &Rgba) -> bool {
        *self == PreRgba::from(*rgba)
    }
}

impl PartialEq<PreRgba> for PreRgba {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        self.3 == pre_rgba.3 && self.0 == pre_rgba.0 && self.1 == pre_rgba.1 && self.2 == pre_rgba.2
    }
}

over!(self,
    Over<bottom: Rgb , Rgb> for  PreRgba {
        let contr_alpha = self.contr_alpha_f64();
        Rgb(
            self.0 + (bottom.0 as f64 * contr_alpha).round() as u8,
            self.1 + (bottom.1 as f64 * contr_alpha).round() as u8,
            self.2 + (bottom.2 as f64 * contr_alpha).round() as u8,
        )
    }
    Over<bottom:     Rgba, PreRgba> for  PreRgba { self.over(PreRgba::from(bottom)) }
    Over<bottom:  PreRgba, PreRgba> for  PreRgba {
        let contr_alpha = self.contr_alpha_f64();
        PreRgba(
            self.0 + (bottom.0 as f64 * contr_alpha).round() as u8,
            self.1 + (bottom.1 as f64 * contr_alpha).round() as u8,
            self.2 + (bottom.2 as f64 * contr_alpha).round() as u8,
            self.3 + (bottom.3 as f64 * contr_alpha).round() as u8,
        )
    }
    Over<bottom:    &Rgb ,    Rgb > for  PreRgba {   self .over(*bottom) }
    Over<bottom:    &Rgba, PreRgba> for  PreRgba {   self .over(*bottom) }
    Over<bottom: &PreRgba, PreRgba> for  PreRgba {   self .over(*bottom) }
    Over<bottom:     Rgb ,    Rgb > for &PreRgba { (*self).over( bottom) }
    Over<bottom:     Rgba, PreRgba> for &PreRgba { (*self).over( bottom) }
    Over<bottom:  PreRgba, PreRgba> for &PreRgba { (*self).over( bottom) }
    Over<bottom:    &Rgb ,    Rgb > for &PreRgba { (*self).over(*bottom) }
    Over<bottom:    &Rgba, PreRgba> for &PreRgba { (*self).over(*bottom) }
    Over<bottom: &PreRgba, PreRgba> for &PreRgba { (*self).over(*bottom) }

    // Over<bottom: &mut    Rgb , ()> for  PreRgba { *bottom =   self .over(*bottom)        }
    // Over<bottom: &mut    Rgba, ()> for  PreRgba { *bottom =   self .over(*bottom).into() }
    // Over<bottom: &mut PreRgba, ()> for  PreRgba { *bottom =   self .over(*bottom)        }
    // Over<bottom: &mut    Rgb , ()> for &PreRgba { *bottom = (*self).over(*bottom)        }
    // Over<bottom: &mut    Rgba, ()> for &PreRgba { *bottom = (*self).over(*bottom).into() }
    // Over<bottom: &mut PreRgba, ()> for &PreRgba { *bottom = (*self).over(*bottom)        }

    // Over<bottom:     Rgb , ()> for  &mut PreRgba { *self = (*self).over( bottom).into() }
    // Over<bottom:     Rgba, ()> for  &mut PreRgba { *self = (*self).over( bottom)        }
    // Over<bottom:  PreRgba, ()> for  &mut PreRgba { *self = (*self).over( bottom)        }
    // Over<bottom:    &Rgb , ()> for  &mut PreRgba { *self = (*self).over(*bottom).into() }
    // Over<bottom:    &Rgba, ()> for  &mut PreRgba { *self = (*self).over(*bottom)        }
    // Over<bottom: &PreRgba, ()> for  &mut PreRgba { *self = (*self).over(*bottom)        }
);
