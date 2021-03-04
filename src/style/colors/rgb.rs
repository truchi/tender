use super::*;

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

impl Rgb {
    /// Maps `red`, `green` and `blue` components with `f`.
    pub fn map(self, f: impl Fn(u8) -> u8) -> Self {
        Self {
            red:   f(self.red),
            green: f(self.green),
            blue:  f(self.blue),
        }
    }
}

impl From<RgbTuple> for Rgb {
    fn from((red, green, blue): RgbTuple) -> Self {
        Self { red, green, blue }
    }
}

impl From<Rgba> for Rgb {
    fn from(
        Rgba {
            red, green, blue, ..
        }: Rgba,
    ) -> Rgb {
        Self { red, green, blue }
    }
}

impl From<PreRgba> for Rgb {
    fn from(pre_rgba: PreRgba) -> Rgb {
        Rgba::from(pre_rgba).into()
    }
}

impl Color for Rgb {
    fn red(self) -> u8 {
        self.red
    }

    fn green(self) -> u8 {
        self.green
    }

    fn blue(self) -> u8 {
        self.blue
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
        u8::MAX
    }
}
