use crate::grid::*;

/// Indexes for [`Grid::Item`].
///
/// The underlying type to index an item is [`Point`].
///
/// `T: Into<Point>` is an [`Index0D`].
///
/// See [`Index1D`], [`Index2D`].
pub trait Index0D: Clone {
    /// Returns the index as a [`Point`], without bounds checking.
    fn unchecked(self) -> Point;

    /// Returns the index as a [`Point`], or `None` if out of bounds.
    ///
    /// When `Some`, guaranties:
    /// - `point.x < size.x`
    /// - `point.y < size.y`
    fn checked(self, size: impl Into<Size>) -> Option<Point>;
}

impl<T: Into<Point> + Clone> Index0D for T {
    fn unchecked(self) -> Point {
        self.into()
    }

    fn checked(self, size: impl Into<Size>) -> Option<Point> {
        let point = self.into();

        if point < size.into() {
            Some(point)
        } else {
            None
        }
    }
}
