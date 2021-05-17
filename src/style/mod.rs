pub mod attributes;
mod cell;
pub mod color;
mod comp;
// mod cursor;

pub use attributes::*;
pub use cell::*;
pub use color::*;
pub use comp::*;
// pub use cursor::*;

pub trait Over<Bottom> {
    type Output;

    fn over(self, bottom: Bottom) -> Self::Output;
}

pub trait Under<Top> {
    type Output;

    fn under(self, top: Top) -> Self::Output;
}

impl<Top: Over<Bottom>, Bottom> Under<Top> for Bottom {
    type Output = Top::Output;

    fn under(self, top: Top) -> Self::Output {
        top.over(self)
    }
}

pub struct Dedup<T>(pub T, pub T);
