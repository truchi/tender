use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

impl Color for PreRgba {
    color!(self, color: T
        from      { color.pre_rgba() }
        red       { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.0 as f64).round() as _ } }
        green     { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.1 as f64).round() as _ } }
        blue      { if self.3 == 0 { 0 } else { (u8::MAX as f64 / self.3 as f64 * self.2 as f64).round() as _ } }
        pre_red   { self.0 }
        pre_green { self.1 }
        pre_blue  { self.2 }
        alpha     { self.3 }
        rgb {
            let Self(red, green, blue, alpha) = self;

            if alpha == 0 {
                Rgb(0, 0, 0)
            } else {
                let ratio = u8::MAX as f64 / alpha as f64;

                Rgb(
                    (ratio * red as f64).round() as _,
                    (ratio * green as f64).round() as _,
                    (ratio * blue as f64).round() as _,
                )
            }
        }
        rgba {
            let Rgb(red, green, blue) = self.rgb();

            Rgba(red, green, blue, self.3)
        }
        pre_rgba { self }
    );
}

from!(pre_rgba: PreRgba =>
    rgb: Rgb
    rgba: Rgba
);
