use super::*;
use std::fmt::{self, Display, Formatter};

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Color for Rgb {
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

impl TryFrom<Rgba> for Rgb {
    type Error = ();

    fn try_from(rgba: Rgba) -> Result<Rgb, ()> {
        if rgba.is_opaque() {
            Ok(Rgb::hard_from(rgba))
        } else {
            Err(())
        }
    }
}

impl HardFrom<Rgba> for Rgb {
    fn hard_from(rgba: Rgba) -> Rgb {
        debug_assert!(rgba.is_opaque());
        Rgb(rgba.0, rgba.1, rgba.2)
    }
}

impl TryFrom<PreRgba> for Rgb {
    type Error = ();

    fn try_from(pre_rgba: PreRgba) -> Result<Rgb, ()> {
        if pre_rgba.is_opaque() {
            Ok(Rgb::hard_from(pre_rgba))
        } else {
            Err(())
        }
    }
}

impl HardFrom<PreRgba> for Rgb {
    fn hard_from(pre_rgba: PreRgba) -> Rgb {
        debug_assert!(pre_rgba.is_opaque());
        Rgb(pre_rgba.0, pre_rgba.1, pre_rgba.2)
    }
}

impl PartialEq<Rgba> for Rgb {
    fn eq(&self, rgba: &Rgba) -> bool {
        Rgb::try_from(*rgba)
            .map(|rgb| *self == rgb)
            .unwrap_or(false)
    }
}

impl PartialEq<PreRgba> for Rgb {
    fn eq(&self, pre_rgba: &PreRgba) -> bool {
        Rgb::try_from(*pre_rgba)
            .map(|rgb| *self == rgb)
            .unwrap_or(false)
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
