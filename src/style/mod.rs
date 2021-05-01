pub mod attributes;
mod cell;
pub mod color;
// mod cursor;

pub use attributes::*;
pub use cell::*;
pub use color::*;
// pub use cursor::*;

pub trait Over<Bottom, Output> {
    fn over(self, bottom: Bottom) -> Output;
}

pub struct Dedup<T>(pub T, pub T);
