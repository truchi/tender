use super::*;
use std::iter::{Peekable, Repeat, Take};

#[derive(Clone, Debug)]
pub struct Wrap<'a, T: Iterator<Item = Word<'a, U>>, U: Clone = ()> {
    words:  T,
    width:  usize,
    first:  bool,
    index:  usize,
    chars:  Peekable<Chars<'a, U>>,
    spaces: Peekable<Take<Repeat<Char<U>>>>,
    // word_width: usize,
}

impl<'a, T: Iterator<Item = Word<'a, U>>, U: Clone> Wrap<'a, T, U> {
    pub fn new(words: T, width: usize) -> Option<Self> {
        if width == 0 {
            None
        } else {
            let (words, chars, spaces) = Self::first_word(words)?;

            Some(Self {
                words,
                width,
                first: true,
                index: 0,
                chars,
                spaces,
            })
        }
    }

    fn first_word(
        mut words: T,
    ) -> Option<(T, Peekable<Chars<'a, U>>, Peekable<Take<Repeat<Char<U>>>>)> {
        let word = words.next()?;
        let chars = word.chars().peekable();
        let spaces = word.spaces().peekable();

        Some((words, chars, spaces))
    }

    fn next_word(&mut self) -> Option<&mut Self> {
        let word = self.words.next()?;
        self.chars = word.chars().peekable();
        self.spaces = word.spaces().peekable();

        Some(self)
    }

    fn next_line(&mut self) -> Option<&mut Self> {
        while let Some(char) = self.chars.peek() {
            if char.is_newline() {
                return Some(self);
            } else {
                self.chars.next();
            }
        }

        self.next_word()?;
        self.next_line()
    }
}

impl<'a, T: Iterator<Item = Word<'a, U>>, U: Clone> Iterator for Wrap<'a, T, U> {
    type Item = Line<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.chars.peek() {
            if char.is_newline() {
                self.chars.next();
                self.index = 0;
                self.first = false;

                Some(Line::Break)
            } else {
                let width = char.width();

                if self.index + width <= self.width {
                    let char = char.clone();

                    self.chars.next();
                    self.index += width;

                    Some(Line::Char(char))
                } else {
                    self.next_line()?.next()
                }
            }
        } else if let Some(space) = self.spaces.peek() {
            debug_assert!(space.width() == 1);

            if self.index == 0 && !self.first {
                self.next_word()?.next()
            } else if self.index < self.width {
                let space = space.clone();

                self.spaces.next();
                self.index += 1;

                Some(Line::Char(space))
            } else {
                self.next_line()?.next()
            }
        } else {
            self.next_word()?.next()
        }
    }
}
