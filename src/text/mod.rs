mod char;
mod chars;
// mod crop;
mod lines;
mod str;
mod word;
mod words;
// mod wrap;

pub use self::{char::*, str::*};
pub use chars::*;
// pub use crop::*;
pub use lines::*;
pub use word::*;
pub use words::*;
// pub use wrap::*;

// New line
pub const NEWLINE: char = '\n';

// Space
pub const ZERO_WIDTH: char = '\u{200B}'; // width 0
pub const SPACE: char = '\u{0020}'; // width 1
pub const EN_QUAD: char = '\u{2000}'; // width 1
pub const EM_QUAD: char = '\u{2001}'; // width 1
pub const EN_SPACE: char = '\u{2002}'; // width 1
pub const EM_SPACE: char = '\u{2003}'; // width 1
pub const THIRD_EM: char = '\u{2004}'; // width 1
pub const FOURTH_EM: char = '\u{2005}'; // width 1
pub const SIXTH_EM: char = '\u{2006}'; // width 1
pub const FIGURE: char = '\u{2007}'; // width 1
pub const PUNCT: char = '\u{2008}'; // width 1
pub const THIN: char = '\u{2009}'; // width 1
pub const HAIR: char = '\u{200A}'; // width 1
pub const MATH: char = '\u{205F}'; // width 1
pub const IDEO: char = '\u{3000}'; // width 2

// Non breaking space
pub const NOBREAK: char = '\u{00A0}'; // width 1
pub const NARROW_NOBREAK: char = '\u{202F}'; // width 1

/*

LINES: -> &str / WordOrBreak / CharOrBreak
------|
Hello world!|
How u doing?|

LINES CROPPED: -> &str / WordOrBreak / CharOrBreak
------|
Hello |
How u |

WRAP: -> &str / WordOrBreak / CharOrBreak
-----|
Hello|
world!|
How u|
doing?|

WRAP CROPPED: -> &str / WordOrBreak / CharOrBreak
-----|
Hello|
world|
How u|
doing|


BREAK: -> &str / WordOrBreak / CharOrBreak
---|
Hel|
lo |
wor|
ld!|
How|
u d|
oin|
g?|

*/

// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub enum CharOrBreak<T> {
// Char(Char<T>),
// Break,
// }
//
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WordOrBreak<'a, T> {
    Word(Word<'a, T>),
    Break,
}

// ================================================================================ //
// ================================================================================ //
// ================================================================================ //
// ================================================================================ //
// ================================================================================ //
// ================================================================================ //
// ================================================================================ //

/*
pub struct Line<'a, M, T: Iterator<Item = Word<'a, M>>> {
    words: T,
    peek:  Option<Word<'a, M>>,
    width: usize,
    index: usize,
}

impl<'a, M, T: Iterator<Item = Word<'a, M>>> Line<'a, M, T> {
    pub fn new(mut words: T, width: usize) -> Self {
        let peek = words.next();
        Self {
            words,
            peek,
            width,
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.peek.is_none()
    }
}

impl<'a, M, T: Iterator<Item = Word<'a, M>>> Iterator for Line<'a, M, T> {
    type Item = Word<'a, M>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 5 {
            None
        } else {
            self.index += 1;
            let item = self.peek.take();
            self.peek = self.words.next();

            item
        }
    }
}

pub struct Lines<'line, 'word, M, T: Iterator<Item = Word<'word, M>>> {
    line: &'line mut Line<'word, M, T>,
}

impl<'line, 'word, M, T: Iterator<Item = Word<'word, M>>> Lines<'line, 'word, M, T> {
    pub fn new(line: &'line mut Line<'word, M, T>) -> Self {
        line.reset();
        Self { line }
    }

    pub fn next(&mut self) -> Option<&mut Line<'word, M, T>> {
        if self.line.is_empty() {
            None
        } else {
            Some(self.line)
        }
    }

    pub fn wrap<
        U: IntoIterator<IntoIter = T, Item = Word<'word, M>>,
        F: FnMut(&mut Line<'word, M, T>),
    >(
        words: U,
        width: usize,
        mut f: F,
    ) {
        let words = words.into_iter();
        let mut line = Line::new(words, width);

        while let Some(line) = Lines::new(&mut line).next() {
            f(line);
        }
    }
}

pub fn test() {
    let mut line = Line::new(
        vec![
            Word::new("hello", " ", ()),
            Word::new("hello", " ", ()),
            Word::new("hello", " ", ()),
            Word::new("hello", " ", ()),
            Word::new("hello", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
            Word::new("world", " ", ()),
        ]
        .into_iter(),
        10,
    );

    while let Some(line) = Lines::new(&mut line).next() {
        println!("----");
        for i in line {
            println!("{:#?}", i.word);
        }
    }
}
*/
