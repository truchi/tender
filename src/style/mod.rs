pub mod attributes;
pub mod cell;
pub mod colors;
// mod cursor;

pub use attributes::*;
pub use cell::*;
pub use colors::*;
// pub use cursor::*;

pub trait Over<Bottom = Self, Output = Bottom> {
    fn over(self, bottom: Bottom) -> Output;
}
