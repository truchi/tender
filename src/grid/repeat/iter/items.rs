use super::*;
use std::ops::Range;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Items<I> {
    fun:    fn(Point) -> I,
    xstart: usize,
    xend:   usize,
    yend:   usize,
    x:      usize,
    y:      usize,
}

impl<I> Items<I> {
    pub(crate) fn new(fun: fn(Point) -> I, index: Rect) -> Self {
        let Point {
            x: Range {
                start: x,
                end: xend,
            },
            y: Range {
                start: y,
                end: yend,
            },
        } = index;

        Self {
            fun,
            xstart: x,
            xend,
            yend,
            x,
            y,
        }
    }
}

impl<I> Iterator for Items<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x, self.y);

        let (x, y) = if x < self.xend {
            self.x += 1;
            (x, y)
        } else if y < self.yend {
            self.x = self.xstart;
            self.y += 1;
            (x, y)
        } else {
            return None;
        };

        Some((self.fun)(Point { x, y }))
    }
}
