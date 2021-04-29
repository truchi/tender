use crate::grid::*;

/// Indexes for [`GridCols::Cols`]/[`GridRows::Rows`]/[`GridItems::Items`].
///
/// The underlying type to index columns/rows/items is [`Rect`]
/// (`Coord<Range<usize>>`).
///
/// `RangeFull` (implied on both axis),
/// `Coord<X: ToRange, Y: ToRange>`
/// and`(X: ToRange, Y: ToRange)` are [`Index2D`]s.
///
/// See [`Index1D`], [`Index2D`].
pub trait Index2D: Clone {
    /// Returns the index as a [`Rect`], without bounds checking.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size`.  
    /// `Excluded` start bounds and `Included` end bounds may overflow.
    fn unchecked(self, size: impl Into<Size>) -> Rect;

    /// Returns the index as [`Rect`], or `None` if out of bounds.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size`.  
    /// `Excluded` start bounds and `Included` end bounds saturate.
    ///
    /// When `Some`, guaranties on both axis:
    /// - `start <= end`
    /// - `end <= len`
    fn checked(self, size: impl Into<Size>) -> Option<Rect>;
}

impl Index2D for std::ops::RangeFull {
    fn unchecked(self, size: impl Into<Size>) -> Rect {
        let size = size.into();

        Point {
            x: 0..size.x,
            y: 0..size.y,
        }
    }

    fn checked(self, size: impl Into<Size>) -> Option<Rect> {
        Some(Index2D::unchecked(self, size))
    }
}

impl<X: ToRange + Clone, Y: ToRange + Clone> Index2D for Coord<X, Y> {
    fn unchecked(self, size: impl Into<Size>) -> Rect {
        (self.x, self.y).unchecked(size)
    }

    fn checked(self, size: impl Into<Size>) -> Option<Rect> {
        (self.x, self.y).checked(size)
    }
}

impl<X: ToRange + Clone, Y: ToRange + Clone> Index2D for (X, Y) {
    fn unchecked(self, size: impl Into<Size>) -> Rect {
        let size = size.into();

        Point {
            x: ToRange::unchecked(self.0, size.x),
            y: ToRange::unchecked(self.1, size.y),
        }
    }

    fn checked(self, size: impl Into<Size>) -> Option<Rect> {
        let size = size.into();

        Some(Point {
            x: ToRange::checked(self.0, size.x)?,
            y: ToRange::checked(self.1, size.y)?,
        })
    }
}
