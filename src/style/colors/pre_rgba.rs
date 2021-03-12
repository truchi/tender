use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl Color for PreRgba {
    color!(self
        red       { if self.3 == 0 { 0 } else { self.0 * u8::MAX / self.3 } }
        green     { if self.3 == 0 { 0 } else { self.1 * u8::MAX / self.3 } }
        blue      { if self.3 == 0 { 0 } else { self.2 * u8::MAX / self.3 } }
        pre_red   { self.0 }
        pre_green { self.1 }
        pre_blue  { self.2 }
        alpha     { self.3 }
    );
}

impl From<Rgb> for PreRgba {
    fn from(color: Rgb) -> Self {
        Self(color.pre_red(), color.pre_green(), color.pre_blue(), 0)
    }
}

impl From<Rgba> for PreRgba {
    fn from(color: Rgba) -> Self {
        Self(
            color.pre_red(),
            color.pre_green(),
            color.pre_blue(),
            color.alpha(),
        )
    }
}
