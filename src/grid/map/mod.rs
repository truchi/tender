//! A grids that maps elements.

pub mod iter;

use super::*;

/// A grid that maps elements with a function.
///
/// See [`Grid::map()`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Map<T: Grid, I> {
    pub(crate) grid: T,
    pub(crate) fun:  fn(T::Item) -> I,
}

impl<T: Grid, I> WithSize for Map<T, I> {
    fn size(&self) -> Size {
        self.grid.size()
    }
}

impl<T: Grid, I> Grid for Map<T, I> {
    type Item = I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        (self.fun)(self.grid.item_unchecked(index))
    }
}

macro_rules! grid1d {
        ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
            impl<T: $Trait, I> $Trait for Map<T, I> {
                type $Assoc =
                    std::iter::Map<<T::$Assoc as IntoIterator>::IntoIter, fn(T::Item) -> I>;

                unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                    self.grid.$fn(index).into_iter().map(self.fun)
                }
            }
        )* };
    }

macro_rules! grid2d {
        ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
            impl<T: $Trait, I> $Trait for Map<T, I> {
                type $Assoc = iter::Iter2D<<T::$Assoc as IntoIterator>::IntoIter, fn(T::Item) -> I>;

                unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                    Self::$Assoc::new(
                        self.grid.$fn(index).into_iter(),
                        self.fun,
                    )
                }
            }
        )* };
    }

grid1d!(
    GridCol Col col_unchecked
    GridRow Row row_unchecked
);

grid2d!(
    GridCols Cols cols_unchecked
    GridRows Rows rows_unchecked
);

impl<T: GridItems, I> GridItems for Map<T, I> {
    type Items = std::iter::Map<<T::Items as IntoIterator>::IntoIter, fn(T::Item) -> I>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.grid.items_unchecked(index).into_iter().map(self.fun)
    }
}
