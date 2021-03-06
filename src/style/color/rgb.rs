use super::*;

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn alpha(self, alpha: u8) -> Rgba {
        Rgba(self.0, self.1, self.2, alpha)
    }
}

impl Color for Rgb {
    fn get_alpha(self) -> u8 {
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
            Ok(Rgb(rgba.0, rgba.1, rgba.2))
        } else {
            Err(())
        }
    }
}

impl TryFrom<PreRgba> for Rgb {
    type Error = ();

    fn try_from(pre_rgba: PreRgba) -> Result<Rgb, ()> {
        if pre_rgba.is_opaque() {
            Ok(Rgb(pre_rgba.0, pre_rgba.1, pre_rgba.2))
        } else {
            Err(())
        }
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
