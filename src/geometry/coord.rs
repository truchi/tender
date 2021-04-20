use std::{
    cmp::Ordering,
    ops::{Add, Range, Sub},
};

/// Alias of [`Coord`].
pub type Point<X = usize, Y = X> = Coord<X, Y>;

/// Alias of [`Coord`].
pub type Size<X = usize, Y = X> = Coord<X, Y>;

/// Alias of [`Coord<Range<usize>>`].
pub type Rect = Coord<Range<usize>>;

/// An x/y pair.
///
/// See [`Point`], [`Size`], [`Rect`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Coord<X = usize, Y = X> {
    /// The x axis value.
    pub x: X,
    /// The y axis value.
    pub y: Y,
}

impl<X: Add<U>, Y: Add<V>, U, V> Add<Coord<U, V>> for Coord<X, Y> {
    type Output = Coord<X::Output, Y::Output>;

    fn add(self, rhs: Coord<U, V>) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<X: Sub<U>, Y: Sub<V>, U, V> Sub<Coord<U, V>> for Coord<X, Y> {
    type Output = Coord<X::Output, Y::Output>;

    fn sub(self, rhs: Coord<U, V>) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coord {
    pub const MAX: Self = Point {
        x: usize::MAX,
        y: usize::MAX,
    };
    pub const ONE: Self = Point { x: 1, y: 1 };
    pub const ZERO: Self = Point { x: 0, y: 0 };

    /// Creates a [`Rect`] from the [`Point`] `self` with `size`.
    pub fn rect(self, size: Size) -> Rect {
        Rect {
            x: self.x..self.x + size.x,
            y: self.y..self.y + size.y,
        }
    }

    /// Returns the intersecting [`Coord`].
    pub fn min(self, other: Size) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

impl Rect {
    // TODO impl WithSize for Rect/Coord?
    /// Returns the [`Size`] of `self`.
    pub fn size(&self) -> Size {
        let Size { x, y } = self.clone();

        debug_assert!(x.start <= x.end, "Rect {:?} has invalid x range", self);
        debug_assert!(y.start <= y.end, "Rect {:?} has invalid y range", self);

        Size {
            x: x.end - x.start,
            y: y.end - y.start,
        }
    }

    /// Returns the starting [`Point`] of `self`.
    pub fn start(&self) -> Point {
        Point {
            x: self.x.start,
            y: self.y.start,
        }
    }

    /// Returns the ending [`Point`] of `self`.
    pub fn end(&self) -> Point {
        Point {
            x: self.x.end,
            y: self.y.end,
        }
    }

    /// Translates `self` by `coord`.
    pub fn translate(self, coord: Coord) -> Self {
        Self {
            x: self.x.start + coord.x..self.x.end + coord.x,
            y: self.y.start + coord.y..self.y.end + coord.y,
        }
    }

    /// Crops `self` to fit in `((0, 0), size)`.
    pub fn crop(&self, size: Size) -> Self {
        let end = self.end().min(size);
        let start = self.start().min(end);

        debug_assert!(start.x <= end.x);
        debug_assert!(start.y <= end.y);
        Rect {
            x: start.x..end.x,
            y: start.y..end.y,
        }
    }
}

impl<X: PartialOrd, Y: PartialOrd> PartialOrd for Coord<X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x.partial_cmp(&other.x), self.y.partial_cmp(&other.y)) {
            (Some(self_ord), Some(other_ord)) if self_ord == other_ord => Some(self_ord),
            _ => None,
        }
    }
}

impl<X, Y> From<Coord<X, Y>> for (X, Y) {
    fn from(coord: Coord<X, Y>) -> Self {
        (coord.x, coord.y)
    }
}

impl<X, Y> From<(X, Y)> for Coord<X, Y> {
    fn from(coord: (X, Y)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
        }
    }
}
