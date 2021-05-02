mod layer;
// mod canvas;
// mod cell;

pub use layer::*;
// pub use canvas::*;
// pub use cell::*;

use super::*;
use crate::{geometry::*, grid::*, style::*};
use std::{io::Stdout, marker::PhantomData, ops::Deref};

pub trait WithPosition {
    fn position(&self) -> Point;
}

impl<T: Deref<Target = U>, U: WithPosition> WithPosition for T {
    fn position(&self) -> Size {
        self.deref().position()
    }
}

// pub struct Screen<Canvas, Cell> {
// cells:   Canvas,
// stdout:  Stdout,
// phantom: PhantomData<Cell>,
// }

// impl WithSize for Screen {
// fn size(&self) -> Size {
// self.cells.size()
// }
// }

// impl Screen {
// pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame> {
// let rect = rect.checked(self.size())?;
// let position = rect.start();
// let cells = unsafe { (&mut self.cells).crop_unchecked(rect) };
//
// Some(Frame {
// position,
// cells,
// stdout: &mut self.stdout,
// })
// }
// }

// pub struct Frame<'a, T> {
// position: Point,
// frame:    &'a mut T,
// }
// cells:    Crop<&'a mut RowVec1D<Cell<Rgb>>>,
// stdout:   &'a mut Stdout,
