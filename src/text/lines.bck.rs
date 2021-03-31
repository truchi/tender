use super::*;
use std::iter::{Peekable, Repeat, Take};

type LineWord<'a, T> = Option<Option<(Peekable<Chars<'a, T>>, Peekable<Take<Repeat<Char<T>>>>)>>;

#[derive(Clone, Debug)]
pub struct Lines<'a, T, U: Clone = ()> {
    words: T,
    first: bool,
    index: usize,
    word:  LineWord<'a, U>,
}

impl<'a, T: Iterator<Item = WordOrBreak<'a, U>>, U: Clone> Lines<'a, T, U> {
    pub fn new(mut words: T) -> Self {
        Self {
            first: true,
            index: 0,
            word: Self::first_word(&mut words),
            words,
        }
    }

    fn first_word(words: &mut T) -> LineWord<'a, U> {
        match words.next() {
            Some(WordOrBreak::Word(word)) =>
                Some(Some((word.chars().peekable(), word.spaces().peekable()))),
            Some(WordOrBreak::Break) => Some(None),
            None => None,
        }
    }

    fn next_word(&mut self) -> &mut Self {
        self.word = match self.words.next() {
            Some(WordOrBreak::Word(word)) =>
                Some(Some((word.chars().peekable(), word.spaces().peekable()))),
            Some(WordOrBreak::Break) => Some(None),
            None => None,
        };

        self
    }
}

impl<'a, T: Iterator<Item = WordOrBreak<'a, U>>, U: Clone> Iterator for Lines<'a, T, U> {
    type Item = CharOrBreak<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Some((chars, spaces))) = &mut self.word {
            if let Some(char) = chars.peek() {
                let width = char.width();

                let char = char.clone();

                chars.next();
                self.index += width;

                return Some(CharOrBreak::Char(char));
            } else if let Some(space) = spaces.peek() {
                debug_assert!(space.width() == 1);

                if self.index == 0 && !self.first {
                    return self.next_word().next();
                } else {
                    let space = space.clone();

                    spaces.next();
                    self.index += 1;

                    return Some(CharOrBreak::Char(space));
                }
            } else {
                return self.next_word().next();
            }
        } else if let Some(None) = &mut self.word {
            self.index = 0;
            self.first = false;
            self.next_word();

            return Some(CharOrBreak::Break);
        } else {
            return None;
        }
    }
}
