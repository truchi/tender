//! Grids that repeat elements.

pub mod iter;

use super::*;
use std::{iter::Take, ops::Range};

// -------------------------------------------------------------- //
//                                                                //
// *************************** REPEAT *************************** //
//                                                                //
// -------------------------------------------------------------- //

/// Creates a grid that repeats an element.
///
/// See [`repeat_with()`].
///
/// # Example
///
/// ```
/// # use tender::grid::*;
/// let rows = repeat((2, 2), "hello").rows((.., ..)).unwrap();
///
/// for row in rows {
///     for item in row {
///         assert_eq!(item, "hello");
///     }
/// }
/// ```
pub fn repeat<I: Clone>(size: impl Into<Size>, item: I) -> Repeat<I> {
    Repeat {
        size: size.into(),
        item,
    }
}

/// A grid that repeats an element.
///
/// See [`repeat()`], [`repeat_with()`].
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

macro_rules! grid {
    ($(($($lifetime:lifetime $clone:ident)?) $Type:ty)*) => {
        grid!([0D] $(($($lifetime)?) $Type)*);
        grid!([1D]
            $(GridCol Col col_unchecked ($($lifetime $clone)?) $Type)*
            $(GridRow Row row_unchecked ($($lifetime $clone)?) $Type)*
        );
        grid!([2D] x y
            $(GridCols Cols cols_unchecked (Col col_unchecked) y x ($($lifetime)?) $Type)*
            $(GridRows Rows rows_unchecked (Row row_unchecked) x y ($($lifetime)?) $Type)*
        );
        grid!([ITEMS] $(($($lifetime $clone)?) $Type)*);
    };
    ([0D] $(($($lifetime:lifetime)?) $Type:ty)*) => { $(
        impl<$($lifetime,)? I: Clone> Grid for $Type {
            type Item = I;

            unsafe fn item_unchecked(self, _: impl Index0D) -> Self::Item {
                self.item.clone()
            }
        }
    )* };
    ([1D] $(
        $Trait:ident $Assoc:ident $fn:ident
        ($($lifetime:lifetime $clone:ident)?) $Type:ty
    )*) => { $(
        impl<$($lifetime,)? I: Clone> $Trait for $Type {
            type $Assoc = Take<std::iter::Repeat<Self::Item>>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let (_, Range { start, end }) = index.$fn(self.size);

                std::iter::repeat(self.item$(.$clone())?).take(end - start)
            }
        }
    )* };
    ([2D] $x:ident $y:ident $(
        $Trait:ident $Assoc:ident $fn:ident
        ($Item:ident $item:ident)
        $main:ident $cross:ident
        ($($lifetime:lifetime)?) $Type:ty
    )*) => { $(
        impl<$($lifetime,)? I: Clone> $Trait for $Type {
            type $Assoc = Take<std::iter::Repeat<Self::$Item>>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                let Point { $x, $y } = index.unchecked(self.size);

                std::iter::repeat(self.$item((0, $main))).take($cross.end - $cross.start)
            }
        }
    )* };
    ([ITEMS] $(
        ($($lifetime:lifetime $clone:ident)?) $Type:ty
    )*) => { $(
        impl<$($lifetime,)? I: Clone> GridItems for $Type {
            type Items = Take<std::iter::Repeat<Self::Item>>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                let Point { x, y } = index.unchecked(self.size);

                std::iter::repeat(self.item$(.$clone())?).take((x.end - x.start) * (y.end - y.start))
            }
        }
    )* }
}

grid!(
    ()                 Repeat<I>
    ('a clone) &'a     Repeat<I>
    ('a clone) &'a mut Repeat<I>
);

// ------------------------------------------------------------------- //
//                                                                     //
// *************************** REPEAT WITH *************************** //
//                                                                     //
// ------------------------------------------------------------------- //

/// Creates a grid that repeats elements by applying the provided function.
///
/// See [`repeat()`].
///
/// # Example
///
/// ```
/// # use tender::geometry::*;
/// # use tender::grid::*;
/// let mut rows = repeat_with((2, 2), |Point { x, y }| if x == y { 1 } else { 0 })
///     .rows((.., ..))
///     .unwrap();
///
/// let mut row1 = rows.next().unwrap();
/// assert_eq!(row1.next(), Some(1));
/// assert_eq!(row1.next(), Some(0));
/// assert_eq!(row1.next(), None);
///
/// let mut row2 = rows.next().unwrap();
/// assert_eq!(row2.next(), Some(0));
/// assert_eq!(row2.next(), Some(1));
/// assert_eq!(row2.next(), None);
///
/// assert_eq!(rows.next(), None);
/// ```
pub fn repeat_with<I>(size: impl Into<Size>, fun: fn(Point) -> I) -> RepeatWith<I> {
    RepeatWith {
        size: size.into(),
        fun,
    }
}

/// A grid that repeats elements by applying the provided function.
///
/// See [`repeat_with()`], [`repeat()`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RepeatWith<I> {
    size: Size,
    fun:  fn(Point) -> I,
}

impl<I> WithSize for RepeatWith<I> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<I> Grid for RepeatWith<I> {
    type Item = I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        (self.fun)(index.unchecked())
    }
}

macro_rules! grid1d {
    ($($Trait:ident<$M:ident> $Assoc:ident $fn:ident)*) => { $(
        impl<I> $Trait for RepeatWith<I> {
            type $Assoc = iter::Iter1D<$M, I>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.$fn(self.size))
            }
        }
    )* };
}

macro_rules! grid2d {
    ($($Trait:ident<$M:ident> $Assoc:ident $fn:ident)*) => { $(
        impl<I> $Trait for RepeatWith<I> {
            type $Assoc = iter::Iter2D<$M, I>;

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
    GridCols<ColMajor> Cols cols_unchecked
    GridRows<RowMajor> Rows rows_unchecked
);

impl<I> GridItems for RepeatWith<I> {
    type Items = iter::Items<I>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        Self::Items::new(self.fun, index.unchecked(self.size))
    }
}
