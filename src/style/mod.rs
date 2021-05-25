pub mod attributes;
pub mod color;

mod cell;
mod comp;
mod damaged;
mod paint;

pub use attributes::*;
pub use cell::*;
pub use color::*;
pub use comp::*;
pub use damaged::*;
pub use paint::*;

pub trait Over<Bottom> {
    type Output;

    fn over(self, bottom: Bottom) -> Self::Output;
}

impl<Top: Over<Bottom, Output = Bottom> + Copy, Bottom: Copy> Over<&mut Bottom> for &Top {
    type Output = ();

    fn over(self, bottom: &mut Bottom) {
        *bottom = (*self).over(*bottom);
    }
}

impl<Top: Over<Bottom, Output = Top> + Copy, Bottom: Copy> Over<&Bottom> for &mut Top {
    type Output = ();

    fn over(self, bottom: &Bottom) {
        *self = (*self).over(*bottom);
    }
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
