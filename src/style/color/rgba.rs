use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, Default, Hash, Debug)]
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

impl PartialEq<Rgba> for Rgba {
    fn eq(&self, rgba: &Rgba) -> bool {
        self.3 == rgba.3 && self.0 == rgba.0 && self.1 == rgba.1 && self.2 == rgba.2
    }
}

impl PartialEq<PreRgba> for Rgba {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        PreRgba::from(*self) == *pre_rgba
    }
}

over!(self,
    Over<bottom:     Rgb ,    Rgb > for  Rgba { PreRgba::from( self).over( bottom) }
    Over<bottom:     Rgba, PreRgba> for  Rgba { PreRgba::from( self).over( bottom) }
    Over<bottom:  PreRgba, PreRgba> for  Rgba { PreRgba::from( self).over( bottom) }
    Over<bottom:    &Rgb ,    Rgb > for  Rgba { PreRgba::from( self).over(*bottom) }
    Over<bottom:    &Rgba, PreRgba> for  Rgba { PreRgba::from( self).over(*bottom) }
    Over<bottom: &PreRgba, PreRgba> for  Rgba { PreRgba::from( self).over(*bottom) }
    Over<bottom:     Rgb ,    Rgb > for &Rgba { PreRgba::from(*self).over( bottom) }
    Over<bottom:     Rgba, PreRgba> for &Rgba { PreRgba::from(*self).over( bottom) }
    Over<bottom:  PreRgba, PreRgba> for &Rgba { PreRgba::from(*self).over( bottom) }
    Over<bottom:    &Rgb ,    Rgb > for &Rgba { PreRgba::from(*self).over(*bottom) }
    Over<bottom:    &Rgba, PreRgba> for &Rgba { PreRgba::from(*self).over(*bottom) }
    Over<bottom: &PreRgba, PreRgba> for &Rgba { PreRgba::from(*self).over(*bottom) }

    // Over<bottom: &mut    Rgb , ()> for  Rgba { *bottom =   self .over(*bottom)        }
    // Over<bottom: &mut    Rgba, ()> for  Rgba { *bottom =   self .over(*bottom).into() }
    // Over<bottom: &mut PreRgba, ()> for  Rgba { *bottom =   self .over(*bottom)        }
    // Over<bottom: &mut    Rgb , ()> for &Rgba { *bottom = (*self).over(*bottom)        }
    // Over<bottom: &mut    Rgba, ()> for &Rgba { *bottom = (*self).over(*bottom).into() }
    // Over<bottom: &mut PreRgba, ()> for &Rgba { *bottom = (*self).over(*bottom)        }

    // Over<bottom:     Rgb , ()> for  &mut Rgba { *self = (*self).over( bottom).into() }
    // Over<bottom:     Rgba, ()> for  &mut Rgba { *self = (*self).over( bottom).into() }
    // Over<bottom:  PreRgba, ()> for  &mut Rgba { *self = (*self).over( bottom).into() }
    // Over<bottom:    &Rgb , ()> for  &mut Rgba { *self = (*self).over(*bottom).into() }
    // Over<bottom:    &Rgba, ()> for  &mut Rgba { *self = (*self).over(*bottom).into() }
    // Over<bottom: &PreRgba, ()> for  &mut Rgba { *self = (*self).over(*bottom).into() }
);
