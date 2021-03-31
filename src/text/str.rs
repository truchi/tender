use super::*;
use std::iter::{Map, Peekable};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Str<'a, T = ()> {
    pub meta: T,
    pub str:  &'a str,
}

impl<'a, T> Str<'a, T> {
    pub fn new(str: &'a str, meta: T) -> Self {
        Self { meta, str }
    }
}

impl<'a, T: Clone> Str<'a, T> {
    pub fn chars(&self) -> Chars<'a, T> {
        Chars::new(self.str, self.meta.clone())
    }
}

impl<'a, T: Clone> IntoIterator for Str<'a, T> {
    type IntoIter = Chars<'a, T>;
    type Item = Char<T>;

    fn into_iter(self) -> Self::IntoIter {
        Chars::new(self.str, self.meta)
    }
}

#[derive(Clone, Debug)]
pub struct PeekableChars<'a, T = ()> {
    pub meta:  T,
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a, T> PeekableChars<'a, T> {
    pub fn new(str: Str<'a, T>) -> Self {
        let meta = str.meta;
        let chars = str.str.chars().peekable();

        Self { meta, chars }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn skip_spaces(&mut self) -> Option<char> {
        while let Some(char) = self.peek() {
            if char.is_space().is_some() {
                self.next();
            } else {
                return Some(char);
            }
        }

        None
    }

    pub fn skip_line(&mut self) -> Option<char> {
        while let Some(char) = self.peek() {
            if !char.is_newline() {
                self.next();
            } else {
                return Some(char);
            }
        }

        return None;
    }
}

#[derive(Clone, Debug)]
pub struct PeekableStrs<'a, I: Iterator<Item = Str<'a, T>>, T = ()> {
    pub strs: Peekable<Map<I, fn(Str<'a, T>) -> PeekableChars<'a, T>>>,
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T> PeekableStrs<'a, I, T> {
    pub fn new(strs: I) -> Self {
        let strs = strs
            .map((|str| PeekableChars::new(str)) as fn(_) -> _)
            .peekable();

        Self { strs }
    }

    pub fn peek_mut(&mut self) -> Option<&mut PeekableChars<'a, T>> {
        self.strs.peek_mut()
    }

    pub fn next(&mut self) -> Option<PeekableChars<'a, T>> {
        self.strs.next()
    }

    pub fn skip_spaces(&mut self) -> Option<char> {
        while let Some(chars) = self.peek_mut() {
            if let Some(char) = chars.skip_spaces() {
                return Some(char);
            } else {
                self.next();
            }
        }

        None
    }

    pub fn skip_line(&mut self) -> Option<char> {
        while let Some(chars) = self.peek_mut() {
            if let Some(char) = chars.skip_line() {
                return Some(char);
            } else {
                self.next();
            }
        }

        None
    }
}
