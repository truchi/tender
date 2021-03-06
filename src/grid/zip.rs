//! A grid that zips two grids.

use crate::grid::*;
use std::iter::Map;

/// ‘Zips up’ two grids into a single grid of pairs.
///
/// See [`Grid::zip`], [`Grid::zip_at`].
#[derive(Copy, Clone, Default, Debug)]
pub struct Zip<A, B> {
    a:    A,
    b:    B,
    size: Size,
}

impl<A: WithSize, B: WithSize> Zip<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        let size = a.size().min(b.size());

        Self { a, b, size }
    }
}

impl<A: Grid, B: Grid> Zip<Crop<A>, Crop<B>> {
    pub(crate) fn at(a: A, b: B, position: Point) -> Self {
        let rect_b = position.rect(b.size());
        let rect_a = rect_b.crop(a.size());
        let size = rect_a.size();
        let rect_b = Point::ZERO.rect(size);

        // SAFETY: the above guaranties we are in bounds
        debug_assert!(rect_a.clone().checked(a.size()).is_some());
        debug_assert!(rect_b.clone().checked(b.size()).is_some());
        let a = unsafe { a.crop_unchecked(rect_a) };
        let b = unsafe { b.crop_unchecked(rect_b) };

        Zip { a, b, size }
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
            type $Assoc = std::iter::Zip<
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
                std::iter::Zip<
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
    GridCols Cols(Col) cols_unchecked
    GridRows Rows(Row) rows_unchecked
);

impl<A: GridItems, B: GridItems> GridItems for Zip<A, B> {
    type Items =
        std::iter::Zip<<A::Items as IntoIterator>::IntoIter, <B::Items as IntoIterator>::IntoIter>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.a
            .items_unchecked(index.clone())
            .into_iter()
            .zip(self.b.items_unchecked(index))
    }
}
