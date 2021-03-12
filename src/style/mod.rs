pub mod attributes;
pub mod colors;
// mod cursor;
mod styles;

pub use attributes::*;
pub use colors::*;
// pub use cursor::*;
pub use styles::*;

pub trait Styler<Fg, Bg> {
    fn get_foreground(self) -> Foreground<Fg>;
    fn get_background(self) -> Background<Bg>;
    fn get_attributes(self) -> Attributes;
    fn get_weight(self) -> Option<Weight>;
    fn get_slant(self) -> Option<Slant>;
    fn get_underline(self) -> Option<Underline>;
    fn get_strike(self) -> Option<Strike>;
    fn get_overline(self) -> Option<Overline>;
    fn get_invert(self) -> Option<Invert>;
    fn get_blink(self) -> Option<Blink>;
    fn get_border(self) -> Option<Border>;

    fn set_foreground(self, color: Fg) -> Self;
    fn set_background(self, color: Bg) -> Self;
    fn set_attributes(self, attributes: Attributes) -> Self;
    fn set_weight(self, weight: Option<Weight>) -> Self;
    fn set_slant(self, slant: Option<Slant>) -> Self;
    fn set_underline(self, underline: Option<Underline>) -> Self;
    fn set_strike(self, strike: Option<Strike>) -> Self;
    fn set_overline(self, overline: Option<Overline>) -> Self;
    fn set_invert(self, invert: Option<Invert>) -> Self;
    fn set_blink(self, blink: Option<Blink>) -> Self;
    fn set_border(self, border: Option<Border>) -> Self;
}
