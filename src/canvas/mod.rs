mod canvas;
mod cell;

pub use canvas::*;
pub use cell::*;

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

pub trait Layer: GridRows<Item = Cell<Rgba>> + WithPosition {}

pub struct GridLayer<T> {
    pub position: Point,
    pub grid:     T,
}

impl<T: WithSize> WithSize for GridLayer<T> {
    fn size(&self) -> Size {
        self.grid.size()
    }
}

impl<T> WithPosition for GridLayer<T> {
    fn position(&self) -> Point {
        self.position
    }
}

impl<T: Grid> Grid for GridLayer<T> {
    type Item = T::Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        self.grid.item_unchecked(index)
    }
}

impl<T: GridRow> GridRow for GridLayer<T> {
    type Row = T::Row;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        self.grid.row_unchecked(index)
    }
}

impl<T: GridRows> GridRows for GridLayer<T> {
    type Rows = T::Rows;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        self.grid.rows_unchecked(index)
    }
}

impl<T: GridRows<Item = Cell<Rgba>>> Layer for GridLayer<T> {}
