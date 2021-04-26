pub mod attributes;
pub mod colors;
// mod cursor;
mod styles;

pub use attributes::*;
pub use colors::*;
// pub use cursor::*;
pub use styles::*;

pub trait Styler<Fg, Bg> {
    fn get_foreground(self) -> Fg;
    fn get_background(self) -> Bg;
    fn get_attributes(self) -> Attributes;
    fn get_weight(self) -> Weight;
    fn get_slant(self) -> Slant;
    fn get_underline(self) -> Underline;
    fn get_strike(self) -> Strike;
    fn get_overline(self) -> Overline;
    fn get_invert(self) -> Invert;
    fn get_blink(self) -> Blink;
    fn get_border(self) -> Border;

    fn set_foreground(self, foreground: Fg) -> Self;
    fn set_background(self, background: Bg) -> Self;
    fn set_attributes(self, attributes: Attributes) -> Self;
    fn set_weight(self, weight: Weight) -> Self;
    fn set_slant(self, slant: Slant) -> Self;
    fn set_underline(self, underline: Underline) -> Self;
    fn set_strike(self, strike: Strike) -> Self;
    fn set_overline(self, overline: Overline) -> Self;
    fn set_invert(self, invert: Invert) -> Self;
    fn set_blink(self, blink: Blink) -> Self;
    fn set_border(self, border: Border) -> Self;
}
