//! A grid from a 1-dimensional collection.
//!
//! This module provides the [`Grid1D`] type, which wraps a 1D collection to
//! elevate into 2D. A [`Grid1D`] is effectively a slice storing items either
//! column by column (*column-major*) or row by row (*row-major*).
//!
//! Since [`Grid1D`] wraps collections that `AsRef<[I]>`, we can use it with
//! a variety of collections. See our [`Slice1D`], [`Array1D`] and [`Vec1D`]
//! aliases.
//!
//! Though you can use all of the `Grid*` traits immutably, it is impossible to
//! get a mutable 2D iterator along the minor axis: `&mut ColGrid1D` does not
//! implement [`GridRows`] nor `&mut RowGrid1D` does not implement
//! [`GridCols`].
//!
//! Know that iterating along the minor axis is **not CPU cache friendly** and
//! should be avoided. See the excellent
//! [Scott Meyers' talk](https://www.youtube.com/watch?v=WDIkqP4JbkE).

mod index;
pub mod iter;

use crate::grid::*;
use index::*;
use std::{iter::Flatten, marker::PhantomData};

/// A grid from an `array`.
///
/// See [`Grid1D`].  
/// See [`ColArray1D`], [`RowArray1D`].
pub type Array1D<M, I, const L: usize> = Grid1D<M, I, [I; L]>;

/// A grid from a *column-major* `array`.
///
/// See [`Grid1D`].  
/// See [`Array1D`], [`RowArray1D`].
pub type ColArray1D<I, const L: usize> = Array1D<ColMajor, I, L>;

/// A grid from a *row-major* `array`.
///
/// See [`Grid1D`].  
/// See [`Array1D`], [`ColArray1D`].
pub type RowArray1D<I, const L: usize> = Array1D<RowMajor, I, L>;

/// A grid from a `slice`.
///
/// See [`Grid1D`].  
/// See [`ColSlice1D`], [`RowSlice1D`].
pub type Slice1D<'a, M, I> = Grid1D<M, I, &'a [I]>;

/// A grid from a *column-major* `slice`.
///
/// See [`Grid1D`].  
/// See [`Slice1D`], [`RowSlice1D`].
pub type ColSlice1D<'a, I> = Slice1D<'a, ColMajor, I>;

/// A grid from a *row-major* `slice`.
///
/// See [`Grid1D`].  
/// See [`Slice1D`], [`ColSlice1D`].
pub type RowSlice1D<'a, I> = Slice1D<'a, RowMajor, I>;

/// A grid from a `Vec`.
///
/// See [`Grid1D`].  
/// See [`ColVec1D`], [`RowVec1D`].
pub type Vec1D<M, I> = Grid1D<M, I, Vec<I>>;

/// A grid from a *column-major* `Vec`.
///
/// See [`Grid1D`].  
/// See [`Vec1D`], [`RowVec1D`].
pub type ColVec1D<I> = Vec1D<ColMajor, I>;

/// A grid from a *row-major* `Vec`.
///
/// See [`Grid1D`].  
/// See [`Vec1D`], [`ColVec1D`].
pub type RowVec1D<I> = Vec1D<RowMajor, I>;

/// A grid from a *column-major* 1-dimensional collection.
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
/// See [`Grid1D`], [`RowGrid1D`].
pub type ColGrid1D<I, T> = Grid1D<ColMajor, I, T>;

/// A grid from a *row-major* 1-dimensional collection.
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
/// See [`Grid1D`], [`ColGrid1D`].
pub type RowGrid1D<I, T> = Grid1D<RowMajor, I, T>;

/// A grid from a 1-dimensional collection.
///
/// A [`Grid1D<M, I, T>`] has a layout type `M` ([`ColMajor`]/[`RowMajor`]), an
/// item type `I` and a collection type `T` (which is
/// `AsRef<[I]>`/`AsMut<[I]>`).
///
/// You can get an [`Item`](Grid::Item) through the [`Grid`] trait, both
/// immutably and mutably.
///
/// See [`ColGrid1D`], [`RowGrid1D`].  
/// See [`Slice1D`], [`Array1D`], [`Vec1D`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid1D<M, I, T> {
    size:    M,
    items:   T,
    phantom: PhantomData<I>,
}

impl<M: Major, I, T> Grid1D<M, I, T> {
    /// Creates a new [`Grid1D`], without checking size.
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

    /// Creates a new [`Grid1D`] if `len == x * y`, `None` otherwise.
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

    /// Creates a new [`Grid1D`] if `len == x * y`, `None` otherwise.
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

impl<M, I, T: AsRef<[I]>> AsRef<[I]> for Grid1D<M, I, T> {
    fn as_ref(&self) -> &[I] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[I]>> AsMut<[I]> for Grid1D<M, I, T> {
    fn as_mut(&mut self) -> &mut [I] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T> WithSize for Grid1D<M, I, T> {
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
        impl<'a, M: Major, I, T: $As<[I]>> Grid for &'a $($mut)? Grid1D<M, I, T> {
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
                let index = index1d(index.$fn(self.size), self.size);

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
            type Items = Flatten<<Self as $GridMajors>::$Majors>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.$majors(index).flatten()
            }
        }
    };
}

grid!(
    RowGrid1D<RowMajor>
        GridRow<Row> (row_unchecked)
        GridCol<Col> (col_unchecked)
        GridRows<Rows> (rows_unchecked)
        GridCols<Cols> (cols_unchecked)
    ColGrid1D<ColMajor>
        GridCol<Col> (col_unchecked)
        GridRow<Row> (row_unchecked)
        GridCols<Cols> (cols_unchecked)
        GridRows<Rows> (rows_unchecked)
);
