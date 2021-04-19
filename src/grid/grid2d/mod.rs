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

// ========== //
// Grid (ref) //
// ========== //

// Grid (ref)

impl<'a, M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>> Grid for &'a Grid2D<M, I, T, U> {
    type Item = &'a I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let index = M::from(index.unchecked());

        &self.items.as_ref()[index.minor()].as_ref()[index.major()]
    }
}

// Major (ref)

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCol for &'a ColGrid2D<I, T, U> {
    type Col = &'a [I];

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        let (i, range) = index.row_unchecked(self.size);

        self.items
            .as_ref()
            .get_unchecked(i)
            .as_ref()
            .get_unchecked(range)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRow for &'a RowGrid2D<I, T, U> {
    type Row = &'a [I];

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        let (i, range) = index.row_unchecked(self.size);

        self.items
            .as_ref()
            .get_unchecked(i)
            .as_ref()
            .get_unchecked(range)
    }
}

// Minor (ref)

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCol for &'a RowGrid2D<I, T, U> {
    type Col = iter::Minor<'a, RowMajor, I, T, U>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRow for &'a ColGrid2D<I, T, U> {
    type Row = iter::Minor<'a, ColMajor, I, T, U>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new_unchecked(self, index)
    }
}

// Majors (ref)

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCols for &'a ColGrid2D<I, T, U> {
    type Cols = iter::Majors<'a, ColMajor, I, T, U>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRows for &'a RowGrid2D<I, T, U> {
    type Rows = iter::Majors<'a, RowMajor, I, T, U>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new_unchecked(self, index)
    }
}

// Minors (ref)

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCols for &'a RowGrid2D<I, T, U> {
    type Cols = iter::Minors<'a, RowMajor, I, T, U>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRows for &'a ColGrid2D<I, T, U> {
    type Rows = iter::Minors<'a, ColMajor, I, T, U>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new_unchecked(self, index)
    }
}

// Items (ref)

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridItems for &'a ColGrid2D<I, T, U> {
    type Items = Flatten<<Self as GridCols>::Cols>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.cols_unchecked(index).flatten()
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridItems for &'a RowGrid2D<I, T, U> {
    type Items = Flatten<<Self as GridRows>::Rows>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.rows_unchecked(index).flatten()
    }
}

// ========== //
// Grid (mut) //
// ========== //

// Grid (mut)

impl<'a, M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>> Grid for &'a mut Grid2D<M, I, T, U> {
    type Item = &'a mut I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let index = M::from(index.unchecked());

        &mut self.items.as_mut()[index.minor()].as_mut()[index.major()]
    }
}

// Major (mut)

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCol for &'a mut ColGrid2D<I, T, U> {
    type Col = &'a mut [I];

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        let (i, range) = index.row_unchecked(self.size);

        self.items
            .as_mut()
            .get_unchecked_mut(i)
            .as_mut()
            .get_unchecked_mut(range)
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRow for &'a mut RowGrid2D<I, T, U> {
    type Row = &'a mut [I];

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        let (i, range) = index.row_unchecked(self.size);

        self.items
            .as_mut()
            .get_unchecked_mut(i)
            .as_mut()
            .get_unchecked_mut(range)
    }
}

// Minor (mut)

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCol for &'a mut RowGrid2D<I, T, U> {
    type Col = iter::MinorMut<'a, RowMajor, I, T, U>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRow for &'a mut ColGrid2D<I, T, U> {
    type Row = iter::MinorMut<'a, ColMajor, I, T, U>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new_unchecked(self, index)
    }
}

// Majors (mut)

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCols for &'a mut ColGrid2D<I, T, U> {
    type Cols = iter::MajorsMut<'a, ColMajor, I, T, U>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::cols_unchecked(self, index)
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRows for &'a mut RowGrid2D<I, T, U> {
    type Rows = iter::MajorsMut<'a, RowMajor, I, T, U>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::rows_unchecked(self, index)
    }
}

// Items (mut)

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridItems for &'a mut ColGrid2D<I, T, U> {
    type Items = Flatten<<Self as GridCols>::Cols>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.cols_unchecked(index).flatten()
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridItems for &'a mut RowGrid2D<I, T, U> {
    type Items = Flatten<<Self as GridRows>::Rows>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.rows_unchecked(index).flatten()
    }
}
