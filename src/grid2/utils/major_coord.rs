use super::*;

pub trait MajorCoord: Copy + From<Coord> + Into<Coord> {
    fn new(major: usize, minor: usize) -> Self;
    fn major(self) -> usize;
    fn minor(self) -> usize;

    fn slice2d_item_index(self, size: Size) -> usize {
        let size = Self::from(size);

        self.minor() * size.major() + self.major()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct ColMajorCoord(pub Coord);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct RowMajorCoord(pub Coord);

impl From<Coord> for ColMajorCoord {
    fn from(coord: Coord) -> Self {
        Self(coord)
    }
}

impl From<Coord> for RowMajorCoord {
    fn from(coord: Coord) -> Self {
        Self(coord)
    }
}

impl From<ColMajorCoord> for Coord {
    fn from(ColMajorCoord(coord): ColMajorCoord) -> Self {
        coord
    }
}

impl From<RowMajorCoord> for Coord {
    fn from(RowMajorCoord(coord): RowMajorCoord) -> Self {
        coord
    }
}

impl MajorCoord for ColMajorCoord {
    fn new(y: usize, x: usize) -> Self {
        Self(Coord { x, y })
    }

    fn major(self) -> usize {
        self.0.y
    }

    fn minor(self) -> usize {
        self.0.x
    }
}

impl MajorCoord for RowMajorCoord {
    fn new(x: usize, y: usize) -> Self {
        Self(Coord { x, y })
    }

    fn major(self) -> usize {
        self.0.x
    }

    fn minor(self) -> usize {
        self.0.y
    }
}
