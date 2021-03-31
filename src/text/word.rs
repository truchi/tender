use super::*;
use std::iter::{repeat, Repeat, Take};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Word<'a, T = ()> {
    pub meta:   T,
    pub word:   Str<'a>,
    pub spaces: usize,
}

impl<'a, T> Word<'a, T> {
    pub fn new(word: &'a str, spaces: usize, meta: T) -> Self {
        let word = Str::new(word, ());

        Self { meta, word, spaces }
    }
}

impl<'a, T: Clone> Word<'a, T> {
    pub fn chars(&self) -> Chars<'a, T> {
        Chars::new(self.word.str, self.meta.clone())
    }

    pub fn spaces(&self) -> Take<Repeat<Char<T>>> {
        repeat(Char::space(self.meta.clone())).take(self.spaces)
    }
}
