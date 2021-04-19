use super::*;

/// A 2D iterator along the major axis of an [`Grid2D`].
#[derive(Debug)]
pub struct Majors<'a, M, I, T, U> {
    grid:  &'a Grid2D<M, I, T, U>,
    index: Rect,
}

impl<'a, M: Major, I, T, U> Majors<'a, M, I, T, U> {
    pub(crate) unsafe fn new_unchecked(grid: &'a Grid2D<M, I, T, U>, index: impl Index2D) -> Self {
        let index = index.unchecked(grid.size);

        Self { grid, index }
    }

    pub(crate) unsafe fn cols_unchecked(grid: &'a Grid2D<M, I, T, U>, index: impl Index2D) -> Self {
        Self::new_unchecked(grid, index)
    }

    pub(crate) unsafe fn rows_unchecked(grid: &'a Grid2D<M, I, T, U>, index: impl Index2D) -> Self {
        Self::new_unchecked(grid, index)
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Iterator for Majors<'a, RowMajor, I, T, U> {
    type Item = &'a [I];

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.y.next()?, self.index.x.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.row_unchecked(index) })
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Iterator for Majors<'a, ColMajor, I, T, U> {
    type Item = &'a [I];

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.x.next()?, self.index.y.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.col_unchecked(index) })
    }
}
