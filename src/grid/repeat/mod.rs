pub mod iter;

use super::*;
use std::{
    iter::{repeat as std_repeat, Repeat as StdRepeat, Take},
    ops::Range,
};

// -------------------------- //
//                            //
// ********* REPEAT ********* //
//                            //
// -------------------------- //

/// Creates a grid that repeats an element all over a [`Size`](Size).
pub fn repeat<I: Clone>(size: Size, item: I) -> Repeat<I> {
    Repeat { size, item }
}

/// A grid that repeats an element all over a [`Size`](Size).
///
/// This `struct` is created by [`repeat`](repeat::repeat()).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Repeat<I> {
    size: Size,
    item: I,
}

impl<I> WithSize for Repeat<I> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I: Clone> Grid for Repeat<I> {
    type Item = I;

    unsafe fn item_unchecked(self, _: impl Index0D) -> Self::Item {
        self.item.clone()
    }
}

macro_rules! grid1d {
    ($($Trait:ident $Assoc:ident $fn:ident)*) => { $(
        impl<I: Clone> $Trait for Repeat<I> {
            type $Assoc = Take<StdRepeat<I>>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let (_, Range { start, end }) = index.$fn(self.size);

                std_repeat(self.item).take(end - start)
            }
        }
    )* };
}

macro_rules! grid2d {
    ($x:ident $y:ident $(
        $Trait:ident $Assoc:ident $fn:ident
        ($Item:ident $item:ident)
        $main:ident $cross:ident
    )*) => { $(
        impl<I: Clone> $Trait for Repeat<I> {
            type $Assoc = Take<StdRepeat<Self::$Item>>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                let Point { $x, $y } = index.unchecked(self.size);

                std_repeat(self.$item((0, $main))).take($cross.end - $cross.start)
            }
        }
    )* };
}

grid1d!(
    GridCol Col col_unchecked
    GridRow Row row_unchecked
);

grid2d!(x y
    GridCols Cols cropped_cols_unchecked (Col col_unchecked) y x
    GridRows Rows cropped_rows_unchecked (Row row_unchecked) x y
);

impl<I: Clone> GridItems for Repeat<I> {
    type Items = Take<StdRepeat<Self::Item>>;

    unsafe fn cropped_items_unchecked(self, index: impl Index2D) -> Self::Items {
        let Point { x, y } = index.unchecked(self.size);

        std_repeat(self.item).take((x.end - x.start) * (y.end - y.start))
    }
}

// ------------------------------- //
//                                 //
// ********* REPEAT WITH ********* //
//                                 //
// ------------------------------- //

/// Creates a grid that repeats elements of type `I` all over a [`Size`](Size)
/// by applying the provided closure.
pub fn repeat_with<I, F: FnMut(Size) -> I>(size: Size, fun: F) -> RepeatWith<F> {
    RepeatWith { size, fun }
}

/// A grid that repeats elements of type `I` all over a [`Size`](Size)
/// by applying the provided closure.
#[derive(Copy, Clone, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct RepeatWith<F> {
    size: Size,
    fun:  F,
}

impl<F> WithSize for RepeatWith<F> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I, F: FnMut(Point) -> I> Grid for RepeatWith<F> {
    type Item = I;

    unsafe fn item_unchecked(mut self, index: impl Index0D) -> Self::Item {
        (self.fun)(index.unchecked())
    }
}

macro_rules! grid1d {
    ($($Trait:ident<$M:ident> $Assoc:ident $fn:ident)*) => { $(
        impl<I, F: FnMut(Point) -> I> $Trait for RepeatWith<F> {
            type $Assoc = iter::Iter1D<$M, F>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.$fn(self.size))
            }
        }
    )* };
}

macro_rules! grid2d {
    ($($Trait:ident<$M:ident> $Assoc:ident $fn:ident)*) => { $(
        impl<I, F: Clone + Fn(Point) -> I> $Trait for RepeatWith<F> {
            type $Assoc = iter::Iter2D<$M, F>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.unchecked(self.size))
            }
        }
    )* };
}

grid1d!(
    GridCol<ColMajor> Col col_unchecked
    GridRow<RowMajor> Row row_unchecked
);

grid2d!(
    GridCols<ColMajor> Cols cropped_cols_unchecked
    GridRows<RowMajor> Rows cropped_rows_unchecked
);

impl<I, F: FnMut(Point) -> I> GridItems for RepeatWith<F> {
    type Items = iter::Items<F>;

    unsafe fn cropped_items_unchecked(self, index: impl Index2D) -> Self::Items {
        Self::Items::new(self.fun, index.unchecked(self.size))
    }
}
