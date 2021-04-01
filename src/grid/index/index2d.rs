use crate::grid::*;
use std::ops::RangeBounds;

/// Indexes for [`GridCols::Cols`]/[`GridRows::Rows`]/[`GridItems::Items`].
///
/// The underlying type to index columns/rows/items is [`Rect`]
/// (`Coord<Range<usize>>`).
///
/// `RangeFull` (implied on both axis), `Coord<X: RangeBounds<usize>, Y:
/// RangeBounds<usize>>` and `(X: RangeBounds<usize>, Y: RangeBounds<usize>)`
/// are [`Index2D`]s.
///
/// See [`Index1D`], [`Index2D`].
pub trait Index2D: Clone {
    /// Returns the index as a [`Rect`], without bounds checking.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size`.  
    /// `Excluded` start bounds and `Included` end bounds may overflow.
    fn unchecked(self, size: Size) -> Rect;

    /// Returns the index as [`Rect`], or `None` if out of bounds.
    ///
    /// `Unbounded` start/end bounds will transform into `0`/`size`.  
    /// `Excluded` start bounds and `Included` end bounds saturate.
    ///
    /// When `Some`, guaranties on both axis:
    /// - `start <= end`
    /// - `end <= len`
    fn checked(self, size: Size) -> Option<Rect>;
}

impl Index2D for std::ops::RangeFull {
    fn unchecked(self, size: Size) -> Rect {
        Point {
            x: 0..size.x,
            y: 0..size.y,
        }
    }

    fn checked(self, size: Size) -> Option<Rect> {
        Some(Index2D::unchecked(self, size))
    }
}

impl<X: RangeBounds<usize> + Clone, Y: RangeBounds<usize> + Clone> Index2D for Coord<X, Y> {
    fn unchecked(self, size: Size) -> Rect {
        (self.x, self.y).unchecked(size)
    }

    fn checked(self, size: Size) -> Option<Rect> {
        (self.x, self.y).checked(size)
    }
}

impl<X: RangeBounds<usize> + Clone, Y: RangeBounds<usize> + Clone> Index2D for (X, Y) {
    fn unchecked(self, size: Size) -> Rect {
        Point {
            x: ToRange::unchecked(self.0, size.x),
            y: ToRange::unchecked(self.1, size.y),
        }
    }

    fn checked(self, size: Size) -> Option<Rect> {
        Some(Point {
            x: ToRange::checked(self.0, size.x)?,
            y: ToRange::checked(self.1, size.y)?,
        })
    }
}
