use std::{cmp::Ordering, ops::Range};

pub type Point = Coord;
pub type Size = Coord;
pub type Rect = Coord<Range<usize>, Range<usize>>;
pub type Col = Coord<usize, Range<usize>>;
pub type Row = Coord<Range<usize>, usize>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Coord<X = usize, Y = X> {
    pub x: X,
    pub y: Y,
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
    fn from(Coord { x, y }: Coord<X, Y>) -> Self {
        (x, y)
    }
}

impl<X, Y> From<(X, Y)> for Coord<X, Y> {
    fn from((x, y): (X, Y)) -> Self {
        Self { x, y }
    }
}
