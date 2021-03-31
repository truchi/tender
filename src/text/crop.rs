use super::*;
use std::iter::{Peekable, Repeat, Take};

type CropWord<'a, T> = Option<Option<(Peekable<Chars<'a, T>>, Peekable<Take<Repeat<Char<T>>>>)>>;

#[derive(Clone, Debug)]
pub struct Crop<'a, T: Iterator<Item = WordOrBreak<'a, U>>, U: Clone = ()> {
    words: T,
    width: usize,
    first: bool,
    index: usize,
    word:  CropWord<'a, U>,
}

impl<'a, T: Iterator<Item = WordOrBreak<'a, U>>, U: Clone> Crop<'a, T, U> {
    pub fn new(mut words: T, width: usize) -> Self {
        let word = Self::first_word(&mut words);

        Self {
            words,
            width,
            first: true,
            index: 0,
            word,
        }
    }

    fn first_word(words: &mut T) -> CropWord<'a, U> {
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

    fn next_line(&mut self) -> &mut Self {
        loop {
            match self.words.next() {
                Some(WordOrBreak::Word(_)) => {}
                Some(WordOrBreak::Break) => {
                    self.word = Some(None);
                    break;
                }
                None => {
                    self.word = None;
                    break;
                }
            };
        }

        self
    }
}

impl<'a, T: Iterator<Item = WordOrBreak<'a, U>>, U: Clone> Iterator for Crop<'a, T, U> {
    type Item = CharOrBreak<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Some((chars, spaces))) = &mut self.word {
            if let Some(char) = chars.peek() {
                let width = char.width();

                if self.index + width <= self.width {
                    let char = char.clone();

                    chars.next();
                    self.index += width;

                    return Some(CharOrBreak::Char(char));
                } else {
                    return self.next_line().next();
                }
            } else if let Some(space) = spaces.peek() {
                debug_assert!(space.width() == 1);

                if self.index == 0 && !self.first {
                    return self.next_word().next();
                } else if self.index < self.width {
                    let space = space.clone();

                    spaces.next();
                    self.index += 1;

                    return Some(CharOrBreak::Char(space));
                } else {
                    return self.next_line().next();
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
