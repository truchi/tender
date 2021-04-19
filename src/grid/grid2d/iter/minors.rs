use super::*;

/// A 2D iterator along the minor axis of an [`Grid2D`].
#[derive(Debug)]
pub struct Minors<'a, M, I, T, U> {
    grid:  &'a Grid2D<M, I, T, U>,
    index: Rect,
}

impl<'a, M: Major, I, T: AsRef<[U]>, U> Minors<'a, M, I, T, U> {
    pub(crate) unsafe fn new_unchecked(grid: &'a Grid2D<M, I, T, U>, index: impl Index2D) -> Self {
        let index = index.unchecked(grid.size);

        Self { grid, index }
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Iterator for Minors<'a, RowMajor, I, T, U> {
    type Item = Minor<'a, RowMajor, I, T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.x.next()?, self.index.y.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.col_unchecked(index) })
    }
}

impl<'a, I, T: AsRef<[U]>, U: AsRef<[I]>> Iterator for Minors<'a, ColMajor, I, T, U> {
    type Item = Minor<'a, ColMajor, I, T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = (self.index.y.next()?, self.index.x.clone());

        // SAFETY: users guaranty index is in bounds at construction
        Some(unsafe { self.grid.row_unchecked(index) })
    }
}
