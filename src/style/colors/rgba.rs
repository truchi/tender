use super::*;

/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Color for Rgba {
    color!(self, color: T
        from      { color.rgba() }
        red       { self.0 }
        green     { self.1 }
        blue      { self.2 }
        alpha     { self.3 }
        rgb       { Rgb(self.0, self.1, self.2) }
        rgba      { self }
        pre_rgba  {
            let Self(red, green, blue, alpha) = self;
            let ratio = alpha as f64 / u8::MAX as f64;

            PreRgba(
                (ratio * red as f64).round() as _,
                (ratio * green as f64).round() as _,
                (ratio * blue as f64).round() as _,
                alpha,
            )
        }
    );
}

from!(rgba: Rgba =>
    rgb: Rgb
    pre_rgba: PreRgba
);

impl Over for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: Rgba) -> PreRgba {
        PreRgba::from(self).over(PreRgba::from(bottom))
    }
}

impl Over<PreRgba> for Rgba {
    type Output = PreRgba;

    fn over(self, bottom: PreRgba) -> PreRgba {
        PreRgba::from(self).over(bottom)
    }
}

impl Over<Rgb> for Rgba {
    type Output = Rgb;

    fn over(self, bottom: Rgb) -> Rgb {
        PreRgba::from(self).over(bottom)
    }
}
