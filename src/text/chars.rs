use super::*;

#[derive(Clone, Debug)]
pub struct Chars<'a, T = ()> {
    pub meta:  T,
    pub chars: std::str::Chars<'a>,
}

impl<'a, T> Chars<'a, T> {
    pub fn new(str: &'a str, meta: T) -> Self {
        Self {
            meta,
            chars: str.chars(),
        }
    }

    pub fn next_line(&mut self) {
        while let Some(char) = self.chars.next() {
            if char.is_newline() {
                return;
            }
        }
    }
}

impl<'a, T: Clone> Iterator for Chars<'a, T> {
    type Item = Char<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Char::new(self.chars.next()?, self.meta.clone()))
    }
}
