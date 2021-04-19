use super::*;
use std::{iter::Flatten, marker::PhantomData};

pub mod iter;

pub type ColArray2D<I, T, U> = Array2D<ColMajor, I, T, U>;
pub type RowArray2D<I, T, U> = Array2D<RowMajor, I, T, U>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Array2D<M, I, T, U> {
    size:    M,
    items:   T,
    phantom: PhantomData<(I, U)>,
}

impl<M: Major, I, T, U> Array2D<M, I, T, U> {
    pub fn new_unchecked(size: impl Into<Size>, items: T) -> Self {
        Self {
            size: size.into().into(),
            items,
            phantom: PhantomData,
        }
    }
}

impl<M, I, T: AsRef<[U]>, U> AsRef<[U]> for Array2D<M, I, T, U> {
    fn as_ref(&self) -> &[U] {
        self.items.as_ref()
    }
}

impl<M, I, T: AsMut<[U]>, U> AsMut<[U]> for Array2D<M, I, T, U> {
    fn as_mut(&mut self) -> &mut [U] {
        self.items.as_mut()
    }
}

impl<M: Major, I, T, U> WithSize for Array2D<M, I, T, U> {
    fn size(&self) -> Size {
        self.size.into()
    }
}

// ========== //
// Grid (ref) //
// ========== //

// Grid

impl<'a, M: Major, I, T: AsRef<[U]>, U: AsRef<[I]>> Grid for &'a Array2D<M, I, T, U> {
    type Item = &'a I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let index = M::from(index.unchecked());

        &self.items.as_ref()[index.minor()].as_ref()[index.major()]
    }
}

// Major

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCol for &'a ColArray2D<I, T, U> {
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

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRow for &'a RowArray2D<I, T, U> {
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

// Minor

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCol for &'a RowArray2D<I, T, U> {
    type Col = iter::Minor<'a, RowMajor, I, T, U>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRow for &'a ColArray2D<I, T, U> {
    type Row = iter::Minor<'a, ColMajor, I, T, U>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new_unchecked(self, index)
    }
}

// Majors

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridCols for &'a ColArray2D<I, T, U> {
    type Cols = iter::Majors<'a, ColMajor, I, T, U>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridRows for &'a RowArray2D<I, T, U> {
    type Rows = iter::Majors<'a, RowMajor, I, T, U>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::new_unchecked(self, index)
    }
}

// Items

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridItems for &'a ColArray2D<I, T, U> {
    type Items = Flatten<<Self as GridCols>::Cols>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.cols_unchecked(index).flatten()
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> GridItems for &'a RowArray2D<I, T, U> {
    type Items = Flatten<<Self as GridRows>::Rows>;

    unsafe fn items_unchecked(self, index: impl Index2D) -> Self::Items {
        self.rows_unchecked(index).flatten()
    }
}

// ========== //
// Grid (mut) //
// ========== //

// Grid

impl<'a, M: Major, I, T: AsMut<[U]>, U: AsMut<[I]>> Grid for &'a mut Array2D<M, I, T, U> {
    type Item = &'a mut I;

    unsafe fn item_unchecked(self, index: impl Index0D) -> Self::Item {
        let index = M::from(index.unchecked());

        &mut self.items.as_mut()[index.minor()].as_mut()[index.major()]
    }
}

// Major

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCol for &'a mut ColArray2D<I, T, U> {
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

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRow for &'a mut RowArray2D<I, T, U> {
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

// Minor

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCol for &'a mut RowArray2D<I, T, U> {
    type Col = iter::MinorMut<'a, RowMajor, I, T, U>;

    unsafe fn col_unchecked(self, index: impl Index1D) -> Self::Col {
        Self::Col::new_unchecked(self, index)
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRow for &'a mut ColArray2D<I, T, U> {
    type Row = iter::MinorMut<'a, ColMajor, I, T, U>;

    unsafe fn row_unchecked(self, index: impl Index1D) -> Self::Row {
        Self::Row::new_unchecked(self, index)
    }
}

// Majors

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridCols for &'a mut ColArray2D<I, T, U> {
    type Cols = iter::MajorsMut<'a, ColMajor, I, T, U>;

    unsafe fn cols_unchecked(self, index: impl Index2D) -> Self::Cols {
        Self::Cols::cols_unchecked(self, index)
    }
}

impl<'a, I, T: AsMut<[U]>, U: AsMut<[I]>> GridRows for &'a mut RowArray2D<I, T, U> {
    type Rows = iter::MajorsMut<'a, RowMajor, I, T, U>;

    unsafe fn rows_unchecked(self, index: impl Index2D) -> Self::Rows {
        Self::Rows::rows_unchecked(self, index)
    }
}
