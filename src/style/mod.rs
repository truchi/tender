pub mod attributes;
pub mod colors;
// mod cursor;
mod styles;

pub use attributes::*;
pub use colors::*;
// pub use cursor::*;
pub use styles::*;

pub trait Over<T = Self> {
    type Output;

    fn over(self, other: T) -> Self::Output;
}

impl Over<Rgb> for PreRgba {
    type Output = Rgb;

    fn over(self, other: Rgb) -> Rgb {
        let over = |a, b| a + b * (u8::MAX - self.alpha);

        Rgb {
            red:   over(self.red, other.red),
            green: over(self.green, other.green),
            blue:  over(self.blue, other.blue),
        }
    }
}
