use super::*;

#[derive(Clone, Debug)]
pub struct Words<'a, T = ()> {
    meta:       T,
    str:        &'a str,
    indices:    std::str::CharIndices<'a>,
    word:       usize,
    space:      usize,
    spaces:     usize,
    in_spaces:  bool,
    at_newline: bool,
}

impl<'a, T> Words<'a, T> {
    pub fn new(str: &'a str, meta: T) -> Self {
        Self {
            meta,
            str,
            indices: str.char_indices(),
            word: 0,
            space: 0,
            spaces: 0,
            in_spaces: false,
            at_newline: false,
        }
    }
}

impl<'a, T: Clone> Iterator for Words<'a, T> {
    type Item = WordOrBreak<'a, T>;

    // TODO NBSP outside words?
    // "a{NBSP} b"
    // "a {NBSP}b"
    // "a {NBSP} b"
    fn next(&mut self) -> Option<Self::Item> {
        if self.at_newline {
            self.at_newline = false;
            return Some(WordOrBreak::Break);
        }

        let len = self.str.len();

        while let Some((i, char)) = self.indices.next() {
            if let Some(w) = char.is_space() {
                if !self.in_spaces {
                    self.space = i;
                    self.in_spaces = true;
                }
                self.spaces += w;
            } else if char.is_newline() {
                let word = self.word;
                let space = self.space;
                let spaces = self.spaces;
                let in_spaces = self.in_spaces;

                self.word = i + 1;
                self.space = i + 1;
                self.spaces = 0;
                self.in_spaces = false;
                self.at_newline = true;

                let end = if in_spaces { space } else { i };

                if word != end || spaces > 0 {
                    debug_assert!(word <= end);
                    debug_assert!(end <= len);
                    let str = unsafe { self.str.get_unchecked(word..end) };
                    let word = Word::new(str, spaces, self.meta.clone());

                    return Some(WordOrBreak::Word(word));
                } else {
                    return self.next();
                }
            } else if self.in_spaces {
                let word = self.word;
                let space = self.space;
                let spaces = self.spaces;

                self.word = i;
                self.spaces = 0;
                self.in_spaces = false;

                debug_assert!(word <= space);
                debug_assert!(space <= len);
                let str = unsafe { self.str.get_unchecked(word..space) };
                let word = Word::new(str, spaces, self.meta.clone());

                return Some(WordOrBreak::Word(word));
            }
        }

        if self.word < len {
            let word = self.word;
            self.word = len;

            let end = if self.in_spaces { self.space } else { len };

            debug_assert!(word <= end);
            debug_assert!(end <= len);
            let str = unsafe { self.str.get_unchecked(word..end) };
            let word = Word::new(str, self.spaces, self.meta.clone());

            Some(WordOrBreak::Word(word))
        } else {
            None
        }
    }
}
