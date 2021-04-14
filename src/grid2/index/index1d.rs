use super::*;
use std::ops::Range;

pub trait Index1D: Clone + Sized {
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>);

    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)>;

    fn col_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.y)
    }

    fn col(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.x, size.y)
    }

    fn row_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.x)
    }

    fn row(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.y, size.x)
    }
}
