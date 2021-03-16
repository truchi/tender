use crate::grid::*;
use std::iter::{Map, Zip as StdZip};

/// ‘Zips up’ two grids into a single grid of pairs.
#[derive(Copy, Clone, Default, Debug)]
pub struct Zip<A, B> {
    a:    A,
    b:    B,
    size: Size,
}

impl<A: WithSize, B: WithSize> Zip<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        let Size { x: xa, y: ya } = a.size();
        let Size { x: xb, y: yb } = b.size();

        Self {
            a,
            b,
            size: Size {
                x: xa.min(xb),
                y: ya.min(yb),
            },
        }
    }
}

impl<A: WithSize, B: WithSize> WithSize for Zip<A, B> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<A: Grid, B: Grid> Grid for Zip<A, B> {
    type Item = (A::Item, B::Item);

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        (
            self.a.item_unchecked(index.clone()),
            self.b.item_unchecked(index),
        )
    }
}

macro_rules! grid1d {
    ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        impl<A: $Trait, B: $Trait> $Trait for Zip<A, B> {
            type $Assoc = StdZip<
                <A::$Assoc as IntoIterator>::IntoIter,
                <B::$Assoc as IntoIterator>::IntoIter,
            >;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                self.a
                    .$fn(index.clone())
                    .into_iter()
                    .zip(self.b.$fn(index))
            }
        }
    )* };
}

macro_rules! grid2d {
    ($($Trait:ident $Assoc:ident($Item:ident) $fn:ident)*) => { $(
        impl<A: $Trait, B: $Trait> $Trait for Zip<A, B> {
            type $Assoc = Map<
                StdZip<
                    <A::$Assoc as IntoIterator>::IntoIter,
                    <B::$Assoc as IntoIterator>::IntoIter,
                >,
                fn((A::$Item, B::$Item)) -> Self::$Item,
            >;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                self.a
                    .$fn(index.clone())
                    .into_iter()
                    .zip(self.b.$fn(index))
                    .map(|(a, b)| a.into_iter().zip(b))
            }
        }
    )* };
}

grid1d!(
    GridCol Col col_unchecked
    GridRow Row row_unchecked
);

grid2d!(
    GridCols Cols(Col) cropped_cols_unchecked
    GridRows Rows(Row) cropped_rows_unchecked
);

impl<A: GridItems, B: GridItems> GridItems for Zip<A, B> {
    type Items = StdZip<<A::Items as IntoIterator>::IntoIter, <B::Items as IntoIterator>::IntoIter>;

    unsafe fn cropped_items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.a
            .cropped_items_unchecked(index.clone())
            .into_iter()
            .zip(self.b.cropped_items_unchecked(index))
    }
}
