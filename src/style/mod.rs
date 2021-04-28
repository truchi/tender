pub mod attributes;
pub mod cell;
pub mod color;
// mod cursor;

pub use attributes::*;
pub use cell::*;
pub use color::*;
// pub use cursor::*;

pub trait Over<Bottom = Self, Output = Bottom> {
    fn over(self, bottom: Bottom) -> Output;
}
