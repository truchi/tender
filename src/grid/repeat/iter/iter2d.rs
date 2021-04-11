use super::*;
use std::marker::PhantomData;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Iter2D<M, I> {
    fun:      fn(Point) -> I,
    index:    Rect,
    _phantom: PhantomData<M>,
}

impl<M, I> Iter2D<M, I> {
    pub(crate) fn new(fun: fn(Point) -> I, index: Rect) -> Self {
        Self {
            fun,
            index,
            _phantom: PhantomData,
        }
    }
}

impl<I> Iterator for Iter2D<RowMajor, I> {
    type Item = super::Iter1D<RowMajor, I>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun,
            (self.index.y.next()?, self.index.x.clone()),
        ))
    }
}

impl<I> Iterator for Iter2D<ColMajor, I> {
    type Item = super::Iter1D<ColMajor, I>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::Item::new(
            self.fun,
            (self.index.x.next()?, self.index.y.clone()),
        ))
    }
}
