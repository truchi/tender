//! 2-dimensional slices.
//!
//! This module provides the [`Slice2D`] type, which wraps slices to elevate
//! into 2D. A [`Slice2D`] is effectively a slice storing grid items either
//! column by column (column major) or row by row (row major).
//!
//! Since [`Slice2D`] wraps collections that `AsRef<[I]>`, we can use it with
//! a variety of collections. See our [`Vec2D`] alias.
//!
//! Though you can use all of the `Grid*` traits immutably, it is impossible to
//! get a mutable 2D iterator along the minor axis: `&mut ColSlice2D` does not
//! implement [`GridRows`] nor `&mut RowSlice2D` does not implement
//! [`GridCols`].
//!
//! Know that iterating along the minor axis is **not CPU cache friendly** and
//! should be avoided. See the excellent
//! [Scott Meyers' talk](https://www.youtube.com/watch?v=WDIkqP4JbkE).

mod index;
pub mod iter;

use crate::grid::*;
use index::*;
use std::marker::PhantomData;

/// A grid from a `Vec`. Alias of [`Slice2D<M, I, Vec<T>>`].
///
/// See [`Slice2D`].  
/// See [`ColVec2D`], [`RowVec2D`].
pub type Vec2D<M, I> = Slice2D<M, I, Vec<I>>;

/// A grid from a column-major `Vec`. Alias of [`Vec2D<ColMajor, I>`].
///
/// See [`Slice2D`].  
/// See [`Vec2D`], [`RowVec2D`].
pub type ColVec2D<I> = Slice2D<ColMajor, I, Vec<I>>;

/// A grid from a row-major `Vec`. Alias of [`Vec2D<RowMajor, I>`].
///
/// See [`Slice2D`].  
/// See [`Vec2D`], [`ColVec2D`].
pub type RowVec2D<I> = Slice2D<RowMajor, I, Vec<I>>;

/// A grid from a column-major slice. Alias of [`Slice2D<ColMajor, I, T>`].
///
/// You can get a [`Col`](GridCol::Col)/[`Cols`](GridCols::Cols) through the
/// [`GridCol`]/[`GridCols`] traits, both immutably and mutably.
///
/// You can get a [`Row`](GridRow::Row) through the [`GridRow`] trait, though
/// this is **not CPU cache friendly**, both immutably and mutably.
///
/// You can get [`Rows`](GridRows::Rows) through the [`GridRows`] trait, though
/// this is **not CPU cache friendly**, but only immutably.
///
/// You can get [`Items`](GridItems::Items) through the [`GridItems`] trait,
/// both immutably and mutably. Items will be yielded in a column-major fashion.
///
/// See [`Slice2D`], [`RowSlice2D`].
pub type ColSlice2D<I, T> = Slice2D<ColMajor, I, T>;

/// A grid from a row-major slice. Alias of [`Slice2D<RowMajor, I, T>`].
///
/// You can get a [`Row`](GridRow::Row)/[`Rows`](GridRows::Rows) through the
/// [`GridRow`]/[`GridRows`] traits, both immutably and mutably.
///
/// You can get a [`Col`](GridCol::Col) through the [`GridCol`] trait, though
/// this is **not CPU cache friendly**, both immutably and mutably.
///
/// You can get [`Cols`](GridCols::Cols) through the [`GridCols`] trait, though
/// this is **not CPU cache friendly**, but only immutably.
///
/// You can get [`Items`](GridItems::Items) through the [`GridItems`] trait,
/// both immutably and mutably. Items will be yielded in a row-major fashion.
///
/// See [`Slice2D`], [`ColSlice2D`].
pub type RowSlice2D<I, T> = Slice2D<RowMajor, I, T>;

/// A grid from a slice.
///
/// A [`Slice2D<M, I, T>`] has a layout type `M` ([`ColMajor`]/[`RowMajor`]), an
/// item type `I` and a collection type `T` (which is
/// `AsRef<[I]>`/`AsMut<[I]>`).
///
/// You can get an [`Item`](Grid::Item) through the [`Grid`] trait, both
/// immutably and mutably.
///
/// See [`ColSlice2D`], [`RowSlice2D`].  
/// See [`Vec2D`], [`ColVec2D`], [`RowVec2D`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Slice2D<M, I, T> {
    size:    M,
    items:   T,
    phantom: PhantomData<I>,
}

impl<M: Major, I, T> Slice2D<M, I, T> {
    /// Creates a new [`Slice2D`], without checking size.
    ///
    /// ### Safety
    ///
    /// Accessing items is *undefined behavior* if `len != x * y`.
    pub unsafe fn new_unchecked<S: Into<Size>>(size: S, items: T) -> Self {
        Self {
            size: size.into().into(),
            items,
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Slice2D`] if `len == x * y`, `None` otherwise.
    pub fn new<S: Into<Size>>(size: S, items: T) -> Option<Self>
    where
        T: AsRef<[I]>,
    {
        let size = size.into();

        if items.as_ref().len() == size.x * size.y {
            // SAFETY: len == x * y
            Some(unsafe { Self::new_unchecked(size, items) })
        } else {
            None
        }
    }

    /// Creates a new [`Slice2D`] if `len == x * y`, `None` otherwise.
    pub fn new_mut<S: Into<Size>>(size: S, mut items: T) -> Option<Self>
    where
        T: AsMut<[I]>,
    {
        let size = size.into();

        if items.as_mut().len() == size.x * size.y {
            // SAFETY: len == x * y
            Some(unsafe { Self::new_unchecked(size, items) })
        } else {
            None
        }
    }

    /// Returns the underlying item collection.
    pub fn into_inner(self) -> T {
        self.items
    }
}

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Slice2D<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Slice2D<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T> WithSize for Slice2D<M, I, T> {
    fn size(&self) -> Size {
        self.size.into()
    }
}

macro_rules! grid {
    ($(
        $Type:ident<$M:ident>
            $GridMajor:ident<$Major:ident> ($major:ident)
            $GridMinor:ident<$Minor:ident> ($minor:ident)
            $GridMajors:ident<$Majors:ident> ($majors:ident)
            $GridMinors:ident<$Minors:ident> ($minors:ident)
    )*) => {
        grid!(impl [ITEM] AsRef as_ref get_unchecked);
        grid!(impl [ITEM] AsMut as_mut get_unchecked_mut (mut));

        $(
            // Major
            grid!(impl [SLICE] $Type $GridMajor $Major $major AsRef as_ref get_unchecked);
            grid!(impl [SLICE] $Type $GridMajor $Major $major AsMut as_mut get_unchecked_mut (mut));

            // Minor
            grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsRef Index1D Minor);
            grid!(impl [ITER] $Type $M $GridMinor $Minor $minor AsMut Index1D MinorMut (mut));

            // Majors
            grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsRef Index2D Majors);
            grid!(impl [ITER] $Type $M $GridMajors $Majors $majors AsMut Index2D MajorsMut (mut));

            // Minors
            grid!(impl [ITER] $Type $M $GridMinors $Minors $minors AsRef Index2D Minors);

            // Items
            grid!(impl [ITEMS] $Type $GridMajors $Majors $majors AsRef);
            grid!(impl [ITEMS] $Type $GridMajors $Majors $majors AsMut (mut));
        )*
    };
    (impl [ITEM] $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, M: Major, I, T: $As<[I]>> Grid for &'a $($mut)? Slice2D<M, I, T> {
            type Item = &'a $($mut)? I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                let index = index0d(index.unchecked(), self.size);

                self.items.$as().$get(index)
            }
        }
    };
    (impl [SLICE] $Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = &'a $($mut)? [I];

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let index = index1d(index.$fn(self.size()), self.size);

                self.items.$as().$get(index)
            }
        }
    };
    (impl [ITER]
        $Type:ident $M:ident
        $Trait:ident $Assoc:ident $fn:ident
        $As:ident
        $Index:ident
        $Iter:ident
        $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T> {
            type $Assoc = iter::$Iter<'a, $M, I, T>;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::new_unchecked(self, index)
            }
        }
    };
    (impl [ITEMS]
        $Type:ident
        $GridMajors:ident $Majors:ident $majors:ident
        $As:ident $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[I]>> GridItems for &'a $($mut)? $Type<I, T> {
            type Items = std::iter::Flatten<<Self as $GridMajors>::$Majors>;

            unsafe fn cropped_items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.$majors(index).flatten()
            }
        }
    };
}

grid!(
    RowSlice2D<RowMajor>
        GridRow<Row> (row_unchecked)
        GridCol<Col> (col_unchecked)
        GridRows<Rows> (cropped_rows_unchecked)
        GridCols<Cols> (cropped_cols_unchecked)
    ColSlice2D<ColMajor>
        GridCol<Col> (col_unchecked)
        GridRow<Row> (row_unchecked)
        GridCols<Cols> (cropped_cols_unchecked)
        GridRows<Rows> (cropped_rows_unchecked)
);
