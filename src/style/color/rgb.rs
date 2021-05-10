use super::*;
use std::fmt::{self, Display, Formatter};

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl WithAlpha for Rgb {
    fn alpha(self) -> u8 {
        u8::MAX
    }

    fn is_opaque(self) -> bool {
        true
    }

    fn is_transparent(self) -> bool {
        false
    }

    fn is_visible(self) -> bool {
        true
    }

    fn is_invisible(self) -> bool {
        false
    }
}

impl From<Rgba> for Rgb {
    fn from(rgba: Rgba) -> Rgb {
        Rgb(rgba.0, rgba.1, rgba.2)
    }
}

impl From<PreRgba> for Rgb {
    fn from(pre_rgba: PreRgba) -> Rgb {
        if let Some(inv_alpha) = pre_rgba.inv_alpha_f64() {
            Rgb(
                (pre_rgba.0 as f64 * inv_alpha).round() as _,
                (pre_rgba.1 as f64 * inv_alpha).round() as _,
                (pre_rgba.2 as f64 * inv_alpha).round() as _,
            )
        } else {
            Rgb(0, 0, 0)
        }
    }
}

impl PartialEq<Rgba> for Rgb {
    fn eq(&self, rgba: &Rgba) -> bool {
        Rgba::from(*self) == *rgba
    }
}

impl PartialEq<PreRgba> for Rgb {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        PreRgba::from(*self) == *pre_rgba
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "2;{};{};{}", self.0, self.1, self.2)
    }
}

impl Over<Rgb> for Rgb {
    type Output = Rgb;

    fn over(self, _: Rgb) -> Rgb {
        self
    }
}

impl Over<Rgba> for Rgb {
    type Output = Rgb;

    fn over(self, _: Rgba) -> Rgb {
        self
    }
}

impl Over<PreRgba> for Rgb {
    type Output = Rgb;

    fn over(self, _: PreRgba) -> Rgb {
        self
    }
}
