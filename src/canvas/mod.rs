mod layer;
// mod canvas;
// mod cell;

pub use layer::*;
// pub use canvas::*;
// pub use cell::*;

use crate::{geometry::*, grid::*, style::*};
use std::ops::Deref;

pub trait WithPosition {
    fn position(&self) -> Point;
}

impl<T: Deref<Target = U>, U: WithPosition> WithPosition for T {
    fn position(&self) -> Size {
        self.deref().position()
    }
}
