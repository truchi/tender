//! A grid from a 2-dimensional collection.
//!
//! This module provides the [`Grid2D`] type, which wraps a 2D collection.
//! A [`Grid2D`] is effectively a slice of slices storing items either column by
//! column (*column-major*) or row by row (*row-major*).
//!
//! Since [`Grid2D`] wraps collections that `AsRef<[AsRef<[I]>]>`, we can use it
//! with a variety of collections. See our [`Array2D`] and [`Vec2D`] aliases.
//!
//! Though you can use all of the `Grid*` traits immutably, it is impossible to
//! get a mutable 2D iterator along the minor axis: `&mut ColGrid2D` does not
//! implement [`GridRows`] nor `&mut RowGrid2D` does not implement
//! [`GridCols`].
//!
//! Know that iterating along the minor axis is **not CPU cache friendly** and
//! should be avoided. See the excellent
//! [Scott Meyers' talk](https://www.youtube.com/watch?v=WDIkqP4JbkE).

use super::*;
use std::{iter::Flatten, marker::PhantomData};

pub mod iter;

/// A grid from an `array` of `array`s.
///
/// See [`Grid2D`].  
/// See [`ColArray2D`], [`RowArray2D`].
pub type Array2D<M, I, const MAJ: usize, const MIN: usize> =
    Grid2D<M, I, [[I; MAJ]; MIN], [I; MAJ]>;

/// A grid from a *column-major* `array` of `array`s.
///
/// See [`Grid2D`].  
/// See [`Array2D`], [`RowArray2D`].
pub type ColArray2D<I, const X: usize, const Y: usize> = Array2D<ColMajor, I, Y, X>;

/// A grid from a *row-major* `array` of `array`s.
///
/// See [`Grid2D`].  
/// See [`Array2D`], [`ColArray2D`].
pub type RowArray2D<I, const X: usize, const Y: usize> = Array2D<ColMajor, I, X, Y>;

/// A grid from a `Vec` of `Vec`s.
///
/// See [`Grid2D`].  
/// See [`ColVec2D`], [`RowVec2D`].
pub type Vec2D<M, I> = Grid2D<M, I, Vec<Vec<I>>, Vec<I>>;

/// A grid from a *column-major* `Vec` of `Vec`s.
///
/// See [`Grid2D`].  
/// See [`Vec2D`], [`RowVec2D`].
pub type ColVec2D<I> = Vec2D<ColMajor, I>;

/// A grid from a *row-major* `Vec` of `Vec`s.
///
/// See [`Grid2D`].  
/// See [`Vec2D`], [`ColVec2D`].
pub type RowVec2D<I> = Vec2D<RowMajor, I>;

/// A grid from a *column-major* 2-dimensional collection.
///
/// See [`Grid2D`], [`RowGrid2D`].
pub type ColGrid2D<I, T, U> = Grid2D<ColMajor, I, T, U>;

/// A grid from a *row-major* 2-dimensional collection.
///
/// See [`Grid2D`], [`ColGrid2D`].
pub type RowGrid2D<I, T, U> = Grid2D<RowMajor, I, T, U>;

/// A grid from a 2-dimensional collection.
///
/// A [`Grid2D<M, I, T, U>`] has a layout type `M` ([`ColMajor`]/[`RowMajor`]),
/// an item type `I`, an outer collection type `T`
/// (which is `AsRef<[U]>`/`AsMut<[U]>`) and an inner collection type `U`
/// (which is `AsRef<[I]>`/`AsMut<[I]>`).
///
/// You can get an [`Item`](Grid::Item) through the [`Grid`] trait, both
/// immutably and mutably.
///
/// See [`ColGrid2D`], [`RowGrid2D`].  
/// See [`Array2D`], [`Vec2D`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid2D<M, I, T, U> {
    size:    M,
    items:   T,
    phantom: PhantomData<(I, U)>,
}

impl<M: Major, I, T, U> Grid2D<M, I, T, U> {
    pub fn new_unchecked(size: impl Into<Size>, items: T) -> Self {
        Self {
            size: size.into().into(),
            items,
            phantom: PhantomData,
        }
    }
}

impl<M, I, T: AsRef<[U]>, U> AsRef<[U]> for Grid2D<M, I, T, U> {
    fn as_ref(&self) -> &[U] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[U]>, U> AsMut<[U]> for Grid2D<M, I, T, U> {
    fn as_mut(&mut self) -> &mut [U] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T, U> WithSize for Grid2D<M, I, T, U> {
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
        grid!(impl [ITEM] AsRef as_ref);
        grid!(impl [ITEM] AsMut as_mut (mut));

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
    (impl [ITEM] $As:ident $as:ident $(($mut:ident))?) => {
        impl<'a, M: Major, I, T: $As<[U]>, U: $As<[I]>> Grid for &'a $($mut)? Grid2D<M, I, T, U> {
            type Item = &'a $($mut)? I;

            unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
                let index = M::from(index.unchecked());

                &$($mut)? self.items.$as()[index.minor()].$as()[index.major()]
            }
        }
    };
    (impl [SLICE] $Type:ident $Trait:ident $Assoc:ident $fn:ident $As:ident $as:ident $get:ident $(($mut:ident))?) => {
        impl<'a, I, T: $As<[U]>, U: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T, U> {
            type $Assoc = &'a $($mut)? [I];

            unsafe fn $fn(self, index: impl Index1D) -> Self::$Assoc {
                let (i, range) = index.$fn(self.size);

                self.items.$as().$get(i).$as().$get(range)
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
        impl<'a, I, T: $As<[U]>, U: $As<[I]>> $Trait for &'a $($mut)? $Type<I, T, U> {
            type $Assoc = iter::$Iter<'a, $M, I, T, U>;

            unsafe fn $fn(self, index: impl $Index) -> Self::$Assoc {
                Self::$Assoc::$fn(self, index)
            }
        }
    };
    (impl [ITEMS]
        $Type:ident
        $GridMajors:ident $Majors:ident $majors:ident
        $As:ident $(($mut:ident))?
    ) => {
        impl<'a, I, T: $As<[U]>, U: $As<[I]>> GridItems for &'a $($mut)? $Type<I, T, U> {
            type Items = Flatten<<Self as $GridMajors>::$Majors>;

            unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
                self.$majors(index).flatten()
            }
        }
    };
}

grid!(
    RowGrid2D<RowMajor>
        GridRow<Row> (row_unchecked)
        GridCol<Col> (col_unchecked)
        GridRows<Rows> (rows_unchecked)
        GridCols<Cols> (cols_unchecked)
    ColGrid2D<ColMajor>
        GridCol<Col> (col_unchecked)
        GridRow<Row> (row_unchecked)
        GridCols<Cols> (cols_unchecked)
        GridRows<Rows> (rows_unchecked)
);
