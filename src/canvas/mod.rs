mod layer;
// mod canvas;
// mod cell;

pub use layer::*;
// pub use canvas::*;
// pub use cell::*;

use super::*;
use crate::{geometry::*, grid::*, style::*};
use std::{
    io::{self, Stdout, Write},
    marker::PhantomData,
    ops::Deref,
};

pub trait WithPosition {
    fn position(&self) -> Point;
}

impl<T: Deref<Target = U>, U: WithPosition> WithPosition for T {
    fn position(&self) -> Size {
        self.deref().position()
    }
}

pub struct Screen<Canvas> {
    canvas: Canvas,
    stdout: Stdout,
}

impl<Canvas> Screen<Canvas> {
    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Self>>
    where
        Canvas: WithSize,
    {
        let rect = rect.checked(self.canvas.size())?;

        Some(Frame { rect, frame: self })
    }
}

pub struct Frame<'a, T> {
    rect:  Rect,
    // canvas:   Crop<&'a mut Canvas>,
    // stdout:   &'a mut Stdout,
    frame: &'a mut T,
}
// cells:    Crop<&'a mut RowVec1D<Cell<Rgb>>>,
// stdout:   &'a mut Stdout,

impl<'a, T> Frame<'a, T> {
    pub fn frame(&mut self, rect: impl Index2D) -> Option<Frame<Self>>
// where
        // &'a mut T: WithSize,
    {
        // let rect = rect.checked(self.frame.size())?;
        // let position = rect.start();
        // let canvas = unsafe { (canvas).crop_unchecked(rect) };

        Some(Frame {
            rect:  self.rect.clone(),
            frame: self,
        })
    }
}
/*
 */
