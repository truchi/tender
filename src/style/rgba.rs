/// Rgba color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
    pub alpha: u8,
}

impl Rgba {
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self {
            red:   f(self.red),
            green: f(self.green),
            blue:  f(self.blue),
            alpha: self.alpha,
        }
    }
}

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

    /// Places `self` over `other`.
    pub fn over(self, other: Self) -> Self {
        let over = |a, b| a + b * (u8::MAX - self.alpha);

        Self {
            red:   over(self.red, other.red),
            green: over(self.green, other.green),
            blue:  over(self.blue, other.blue),
            alpha: over(self.alpha, other.alpha),
        }
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

impl From<PreRgba> for Rgba {
    fn from(
        PreRgba {
            red,
            green,
            blue,
            alpha,
        }: PreRgba,
    ) -> Self {
        if alpha == 0 {
            Self::default()
        } else {
            Self {
                red,
                green,
                blue,
                alpha,
            }
            .map(|v| v * u8::MAX / alpha)
        }
    }
}
