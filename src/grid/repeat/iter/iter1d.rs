use super::*;
use std::{marker::PhantomData, ops::Range};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Iter1D<M, I> {
    fun:      fn(Point) -> I,
    index:    (usize, Range<usize>),
    _phantom: PhantomData<M>,
}

impl<M, I> Iter1D<M, I> {
    pub(crate) fn new(fun: fn(Point) -> I, index: (usize, Range<usize>)) -> Self {
        Self {
            fun,
            index,
            _phantom: PhantomData,
        }
    }
}

impl<M: Major, I> Iterator for Iter1D<M, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, current) = (self.index.0, self.index.1.next()?);

        Some((self.fun)(M::new(current, i).into()))
    }
}
