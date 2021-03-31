use super::*;
use std::{
    iter::{once, Once},
    ops::Deref,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Char<T = ()> {
    pub meta:  T,
    pub char:  char,
    pub width: usize,
}

impl<T> Char<T> {
    pub fn new(char: char, meta: T) -> Self {
        Self {
            meta,
            char,
            width: char.width(),
        }
    }

    pub fn newline(meta: T) -> Self {
        Self {
            meta,
            char: '\n',
            width: 0,
        }
    }

    pub fn space(meta: T) -> Self {
        Self {
            meta,
            char: ' ',
            width: 1,
        }
    }

    pub fn ellipsis(meta: T) -> Self {
        Self {
            meta,
            char: '…',
            width: 1,
        }
    }

    pub fn nul(meta: T) -> Self {
        Self {
            meta,
            char: '\u{0}',
            width: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn char(&self) -> char {
        self.char
    }

    pub fn is_newline(&self) -> bool {
        self.char == NEWLINE
    }

    pub fn is_space(&self) -> Option<usize> {
        self.char.is_space()
    }
}

impl<T> Deref for Char<T> {
    type Target = char;

    fn deref(&self) -> &char {
        &self.char
    }
}

impl<T> IntoIterator for Char<T> {
    type IntoIter = Once<Char<T>>;
    type Item = Char<T>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}

pub trait CharExt {
    fn is_newline(&self) -> bool;
    fn is_space(&self) -> Option<usize>;
    fn width(&self) -> usize;
}

impl CharExt for char {
    fn is_newline(&self) -> bool {
        *self == NEWLINE
    }

    fn is_space(&self) -> Option<usize> {
        if *self == SPACE
            || *self == EN_QUAD
            || *self == EM_QUAD
            || *self == EN_SPACE
            || *self == EM_SPACE
            || *self == THIRD_EM
            || *self == FOURTH_EM
            || *self == SIXTH_EM
            || *self == FIGURE
            || *self == PUNCT
            || *self == THIN
            || *self == HAIR
            || *self == MATH
        {
            Some(1)
        } else if *self == ZERO_WIDTH {
            Some(0)
        } else if *self == IDEO {
            Some(2)
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        use unicode_width::UnicodeWidthChar;
        UnicodeWidthChar::width(*self).unwrap_or(0)
    }
}
