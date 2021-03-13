use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl Color for PreRgba {
    color!(self
        red       { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.0 as f64).round() as _ } }
        green     { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.1 as f64).round() as _ } }
        blue      { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.2 as f64).round() as _ } }
        pre_red   { self.0 }
        pre_green { self.1 }
        pre_blue  { self.2 }
        alpha     { self.3 }
    );
}

impl From<Rgb> for PreRgba {
    fn from(color: Rgb) -> Self {
        Rgba::from(color).into()
    }
}

impl From<Rgba> for PreRgba {
    fn from(Rgba(red, green, blue, alpha): Rgba) -> Self {
        let ratio = alpha as f64 / u8::MAX as f64;

        Self(
            (ratio * red as f64).round() as _,
            (ratio * green as f64).round() as _,
            (ratio * blue as f64).round() as _,
            alpha,
        )
    }
}
