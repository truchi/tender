//! Iterators for [`Map`](super::Map)'s `Grid*` implementations.

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Iter2D<T, F> {
    iter: T,
    fun:  F,
}

impl<T, F> Iter2D<T, F> {
    pub(crate) fn new(iter: T, fun: F) -> Self {
        Self { iter, fun }
    }
}

type Item<T> = <T as Iterator>::Item;
type Iter<T> = <<T as Iterator>::Item as IntoIterator>::IntoIter;

impl<T: Iterator, I> Iterator for Iter2D<T, fn(Item<Iter<T>>) -> I>
where
    T::Item: IntoIterator,
{
    type Item = std::iter::Map<Iter<T>, fn(Item<Iter<T>>) -> I>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.into_iter().map(self.fun))
    }
}
