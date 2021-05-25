//! A grid that iterate in a [`Rect`] of itself.

use crate::grid::*;

/// A grid that only iterates in a [`Rect`].
///
/// See [`Grid::crop()`].
#[derive(Clone, Default, Debug)]
pub struct Crop<T> {
    rect: Rect,
    grid: T,
}

impl<T: WithSize> Crop<T> {
    pub(crate) fn new(rect: impl Index2D, grid: T) -> Option<Self> {
        let rect = rect.checked(grid.size())?;

        Some(Self { rect, grid })
    }

    pub(crate) unsafe fn new_unchecked(rect: impl Index2D, grid: T) -> Self {
        let rect = rect.unchecked(grid.size());

        Self { rect, grid }
    }
}

impl<T> WithSize for Crop<T> {
    fn size(&self) -> Size {
        self.rect.size()
    }
}

impl<T: Grid> Grid for Crop<T> {
    type Item = T::Item;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let mut index = index.unchecked();
        index.x += self.rect.x.start;
        index.y += self.rect.y.start;

        self.grid.item_unchecked(index)
    }
}

macro_rules! grid1d {
    ($($Trait:ident $Assoc:ident $fn:ident $i:ident $range:ident)*) => { $(
        impl<T: $Trait> $Trait for Crop<T> {
            type $Assoc = T::$Assoc;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let mut index = index.$fn(self.size());
                index.0 += self.rect.$i.start;
                index.1.start += self.rect.$range.start;
                index.1.end += self.rect.$range.start;

                self.grid.$fn(index)
            }
        }
    )* };
}

macro_rules! grid2d {
    ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        impl<T: $Trait> $Trait for Crop<T> {
            type $Assoc = T::$Assoc;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                let mut index = index.unchecked(self.size());
                index.x.start += self.rect.x.start;
                index.y.start += self.rect.y.start;
                index.x.end += self.rect.x.start;
                index.y.end += self.rect.y.start;

                self.grid.$fn(index)
            }
        }
    )* };
}

grid1d!(
    GridCol Col col_unchecked x y
    GridRow Row row_unchecked y x
);

grid2d!(
    GridCols  Cols  cols_unchecked
    GridRows  Rows  rows_unchecked
    GridItems Items items_unchecked
);
