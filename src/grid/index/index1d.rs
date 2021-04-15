use crate::grid::*;
use std::ops::Range;

/// Indexes for [`GridCol::Col`]/[`GridRow::Row`].
///
/// The underlying type to index a column/row is `(usize, Range<usize>)`, with:
/// - `usize`: the index of the column/row,
/// - `Range<usize>`: the range of items in that column/row.
///
/// `usize` and `(usize, T: RangeBounds<usize>)` are [`Index1D`]s.
///
/// See [`Index0D`], [`Index2D`].
pub trait Index1D: Clone + Sized {
    /// Returns the index as `(usize, Range<usize>)`, without bounds checking.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`max_end`.  
    /// `Excluded` start bounds and `Included` end bounds may overflow.
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>);

    /// Returns the index as `(usize, Range<usize>)`, or `None` if out of
    /// bounds.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`max_end`.  
    /// `Excluded` start bounds and `Included` end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < max_i`
    /// - `range.start <= range.end`
    /// - `range.end <= end`
    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)>;

    /// Returns the column index as `(usize, Range<usize>)`, without bounds
    /// checking.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size.y`.  
    /// `Excluded` start bounds and `Included` end bounds may overflow.
    fn col_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.y)
    }

    /// Returns the column index as `(usize, Range<usize>)`, or `None` if out of
    /// bounds.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size.y`.  
    /// `Excluded` start bounds and `Included` end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < size.x`
    /// - `range.start <= range.end`
    /// - `range.end <= size.y`
    fn col(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.x, size.y)
    }

    /// Returns the row index as `(usize, Range<usize>)`, without bounds
    /// checking.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size.x`.  
    /// `Excluded` start bounds and `Included` end bounds may overflow.
    fn row_unchecked(self, size: Size) -> (usize, Range<usize>) {
        self.unchecked(size.x)
    }

    /// Returns the row index as `(usize, Range<usize>)`, or `None` if out of
    /// bounds.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size.x`.  
    /// `Excluded` start bounds and `Included` end bounds saturate.
    ///
    /// When `Some`, guaranties:
    /// - `usize < size.y`
    /// - `range.start <= range.end`
    /// - `range.end <= size.x`
    fn row(self, size: Size) -> Option<(usize, Range<usize>)> {
        self.checked(size.y, size.x)
    }
}

impl Index1D for usize {
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self, 0..max_end)
    }

    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        if self < max_i {
            Some(self.unchecked(max_end))
        } else {
            None
        }
    }
}

impl<T: ToRange + Clone> Index1D for (usize, T) {
    fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
        (self.0, ToRange::unchecked(self.1, max_end))
    }

    fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
        let (i, range) = self;

        if i < max_i {
            Some((i, ToRange::checked(range, max_end)?))
        } else {
            None
        }
    }
}

/*
macro_rules! index1d {
    ($($Type:ty)*) => { $(
        impl Index1D for (usize, $Type) {
            fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
                (self.0, ToRange::unchecked(self.1, max_end))
            }

            fn checked(self, max_i: usize, max_end: usize) -> Option<(usize, Range<usize>)> {
                let (i, range) = self;

                if i < max_i {
                    Some((i, ToRange::checked(range, max_end)?))
                } else {
                    None
                }
            }
        }
    )* };
}
*/

// index1d!(
// RangeFull RangeToInclusive<usize> RangeTo<usize>
// RangeFrom<usize> RangeInclusive<usize> Range<usize>
// (Bound<usize>, Bound<usize>)
// );

// impl<T: RangeBounds<usize> + Clone> Index1D for (usize, T) {
// fn unchecked(self, max_end: usize) -> (usize, Range<usize>) {
// (self.0, ToRange::unchecked(self.1, max_end))
// }
//
// fn checked(self, max_i: usize, max_end: usize) -> Option<(usize,
// Range<usize>)> { let (i, range) = self;
//
// if i < max_i {
// Some((i, ToRange::checked(range, max_end)?))
// } else {
// None
// }
// }
// }
