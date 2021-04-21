use super::*;
use std::fmt::{self, Display, Formatter};

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Color for Rgb {
    color!(self, color: T
        from      { color.rgb() }
        red       { self.0 }
        green     { self.1 }
        blue      { self.2 }
        pre_red   { self.0 }
        pre_green { self.1 }
        pre_blue  { self.2 }
        alpha     { u8::MAX }
        rgb       { self }
        rgba      { Rgba(self.0, self.1, self.2, u8::MAX) }
        pre_rgba  { PreRgba(self.0, self.1, self.2, u8::MAX) }
    );
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "2;{};{};{}", self.red(), self.green(), self.blue())
    }
}

from!(rgb: Rgb =>
    rgba: Rgba
    pre_rgba: PreRgba
);
