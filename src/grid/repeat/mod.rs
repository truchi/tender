//! Grids that repeat elements.

pub mod iter;

use super::*;
use iter::*;
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
pub fn repeat<I>(size: impl Into<Size>, item: I) -> Repeat<I> {
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

macro_rules! repeat_grid {
    ([0D] $self:ident $(
        $($lifetime:lifetime)?, $Type:ty, $Item:ty, $item:expr, $($Clone:ident)?,
    )*) => { $(
        impl<$($lifetime,)? I $(: $Clone)?> Grid for $Type {
            type Item = $Item;

            unsafe fn item_unchecked($self, _: impl Index0D) -> Self::Item {
                $item
            }
        }
    )* };
    ([1D] $self:ident $(
        $Trait:ident $Assoc:ident $fn:ident:
        $($lifetime:lifetime)?, $Type:ty, $item:expr, $($Clone:ident)?,
    )*) => { $(
        impl<$($lifetime,)? I $(: $Clone)?> $Trait for $Type {
            type $Assoc = Take<std::iter::Repeat<Self::Item>>;

            unsafe fn $fn($self, index: impl Index1D) -> Self::$Assoc {
                let (_, Range { start, end }) = index.$fn($self.size);

                std::iter::repeat($item).take(end - start)
            }
        }
    )* };
    ([2D] $x:ident $y:ident $(
        $Trait:ident $Assoc:ident $fn:ident
        ($Item:ident $item:ident)
        $main:ident $cross:ident:
        $($lifetime:lifetime)?, $Type:ty, $($Clone:ident)?,
    )*) => { $(
        impl<$($lifetime,)? I $(: $Clone)?> $Trait for $Type {
            type $Assoc = Take<std::iter::Repeat<Self::$Item>>;

            unsafe fn $fn(self, index: impl Index2D) -> Self::$Assoc {
                let Point { $x, $y } = index.unchecked(self.size);

                std::iter::repeat(self.$item((0, $main))).take($cross.end - $cross.start)
            }
        }
    )* };
    ([ITEMS] $self:ident $(
        $($lifetime:lifetime)?, $Type:ty, $item:expr, $($Clone:ident)?,
    )*) => { $(
        impl<$($lifetime,)? I $(: $Clone)?> GridItems for $Type {
            type Items = Take<std::iter::Repeat<Self::Item>>;

            unsafe fn items_unchecked($self, index: impl Index2D) -> Self::Items {
                let Point { x, y } = index.unchecked($self.size);

                std::iter::repeat($item).take((x.end - x.start) * (y.end - y.start))
            }
        }
    )* }
}

repeat_grid!([0D] self
      ,         Repeat<I>,     I,  self.item.clone(), Clone,
    'a, &'a     Repeat<I>, &'a I, &self.item        ,      ,
    'a, &'a mut Repeat<I>, &'a I, &self.item        ,      ,
);
repeat_grid!([1D] self
    GridCol Col col_unchecked:   ,         Repeat<I>,  self.item, Clone,
    GridCol Col col_unchecked: 'a, &'a     Repeat<I>, &self.item,      ,
    GridCol Col col_unchecked: 'a, &'a mut Repeat<I>, &self.item,      ,

    GridRow Row row_unchecked:   ,         Repeat<I>,  self.item, Clone,
    GridRow Row row_unchecked: 'a, &'a     Repeat<I>, &self.item,      ,
    GridRow Row row_unchecked: 'a, &'a mut Repeat<I>, &self.item,      ,
);
repeat_grid!([2D] x y
    GridCols Cols cols_unchecked (Col col_unchecked) y x:   ,         Repeat<I>, Clone,
    GridCols Cols cols_unchecked (Col col_unchecked) y x: 'a, &'a     Repeat<I>,      ,
    GridCols Cols cols_unchecked (Col col_unchecked) y x: 'a, &'a mut Repeat<I>,      ,

    GridRows Rows rows_unchecked (Row row_unchecked) x y:   ,         Repeat<I>, Clone,
    GridRows Rows rows_unchecked (Row row_unchecked) x y: 'a, &'a     Repeat<I>,      ,
    GridRows Rows rows_unchecked (Row row_unchecked) x y: 'a, &'a mut Repeat<I>,      ,
);
repeat_grid!([ITEMS] self
      ,         Repeat<I>,  self.item, Clone,
    'a, &'a     Repeat<I>, &self.item,      ,
    'a, &'a mut Repeat<I>, &self.item,      ,
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

macro_rules! repeat_with_grid {
    ([0D] $(
        $($lifetime:lifetime)?, $Type:ty,
    )*) => { $(
        impl<$($lifetime,)? I> Grid for $Type {
            type Item = I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                (self.fun)(index.unchecked())
            }
        }
    )* };
    ([1D] $(
        $Trait:ident<$M:ident> $Assoc:ident $fn:ident:
        $($lifetime:lifetime)?, $Type:ty,
    )*) => { $(
        impl<$($lifetime,)? I> $Trait for $Type {
            type $Assoc = iter::Iter1D<$M, I>;

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.$fn(self.size))
            }
        }
    )* };
    ([2D] $(
        $Trait:ident $Index:ident $Assoc:ident $fn:ident $Iter:ty:
        $($lifetime:lifetime)?, $Type:ty,
    )*) => { $(
        impl<$($lifetime,)? I> $Trait for $Type {
            type $Assoc = $Iter;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::new(self.fun, index.unchecked(self.size))
            }
        }
    )* };
}

repeat_with_grid!([0D]
      ,         RepeatWith<I>,
    'a, &'a     RepeatWith<I>,
    'a, &'a mut RepeatWith<I>,
);
repeat_with_grid!([1D]
    GridCol<ColMajor> Col col_unchecked:   ,         RepeatWith<I>,
    GridCol<ColMajor> Col col_unchecked: 'a, &'a     RepeatWith<I>,
    GridCol<ColMajor> Col col_unchecked: 'a, &'a mut RepeatWith<I>,

    GridRow<RowMajor> Row row_unchecked:   ,         RepeatWith<I>,
    GridRow<RowMajor> Row row_unchecked: 'a, &'a     RepeatWith<I>,
    GridRow<RowMajor> Row row_unchecked: 'a, &'a mut RepeatWith<I>,
);
repeat_with_grid!([2D]
    GridCols Index2D Cols cols_unchecked Iter2D<ColMajor, I>:   ,         RepeatWith<I>,
    GridCols Index2D Cols cols_unchecked Iter2D<ColMajor, I>: 'a, &'a     RepeatWith<I>,
    GridCols Index2D Cols cols_unchecked Iter2D<ColMajor, I>: 'a, &'a mut RepeatWith<I>,

    GridRows Index2D Rows rows_unchecked Iter2D<RowMajor, I>:   ,         RepeatWith<I>,
    GridRows Index2D Rows rows_unchecked Iter2D<RowMajor, I>: 'a, &'a     RepeatWith<I>,
    GridRows Index2D Rows rows_unchecked Iter2D<RowMajor, I>: 'a, &'a mut RepeatWith<I>,

    GridItems Index2D Items items_unchecked Items<I>:   ,         RepeatWith<I>,
    GridItems Index2D Items items_unchecked Items<I>: 'a, &'a     RepeatWith<I>,
    GridItems Index2D Items items_unchecked Items<I>: 'a, &'a mut RepeatWith<I>,
);
