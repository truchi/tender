use super::*;

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Color for Rgb {
    color!( self
        red       { self.0 }
        green     { self.1 }
        blue      { self.2 }
        pre_red   { self.0 }
        pre_green { self.1 }
        pre_blue  { self.2 }
        alpha     { u8::MAX }
    );
}

impl From<Rgba> for Rgb {
    fn from(color: Rgba) -> Rgb {
        Self(color.red(), color.green(), color.blue())
    }
}

impl From<PreRgba> for Rgb {
    fn from(color: PreRgba) -> Rgb {
        Self(color.red(), color.green(), color.blue())
    }
}
