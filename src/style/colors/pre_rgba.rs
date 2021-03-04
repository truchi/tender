use super::*;

/// Premultiplied-alpha rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct PreRgba {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
    pub alpha: u8,
}

impl PreRgba {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self {
            red:   f(self.red),
            green: f(self.green),
            blue:  f(self.blue),
            alpha: self.alpha,
        }
    }

    // /// Places `self` over `other`.
    // pub fn over(self, other: Self) -> Self {
    //     let over = |a, b| a + b * (u8::MAX - self.alpha);
    //
    //     Self {
    //         red:   over(self.red, other.red),
    //         green: over(self.green, other.green),
    //         blue:  over(self.blue, other.blue),
    //         alpha: over(self.alpha, other.alpha),
    //     }
    // }

    // /// Places `self` over `other`.
    // pub fn over(self, other: Rgb) -> Rgb {
    // let over = |a, b| a + b * (u8::MAX - self.alpha);
    //
    // Rgb {
    // red:   over(self.red, other.red),
    // green: over(self.green, other.green),
    // blue:  over(self.blue, other.blue),
    // }
    // }
}

impl From<RgbTuple> for PreRgba {
    fn from(rgb: RgbTuple) -> Self {
        Rgba::from(rgb).into()
    }
}

impl From<Rgb> for PreRgba {
    fn from(rgb: Rgb) -> Self {
        Rgba::from(rgb).into()
    }
}

impl From<Rgba> for PreRgba {
    fn from(
        Rgba {
            red,
            green,
            blue,
            alpha,
        }: Rgba,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
        .map(|v| v * alpha / u8::MAX)
    }
}

impl Color for PreRgba {
    fn red(self) -> u8 {
        if self.alpha == 0 {
            0
        } else {
            self.red * u8::MAX / self.alpha
        }
    }

    fn green(self) -> u8 {
        if self.alpha == 0 {
            0
        } else {
            self.green * u8::MAX / self.alpha
        }
    }

    fn blue(self) -> u8 {
        if self.alpha == 0 {
            0
        } else {
            self.blue * u8::MAX / self.alpha
        }
    }

    fn pre_red(self) -> u8 {
        self.red
    }

    fn pre_green(self) -> u8 {
        self.green
    }

    fn pre_blue(self) -> u8 {
        self.blue
    }

    fn alpha(self) -> u8 {
        self.alpha
    }
}
