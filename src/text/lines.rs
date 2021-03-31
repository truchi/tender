use super::*;

#[derive(Clone, Debug)]
pub struct Lines<'a, I: Iterator<Item = Str<'a, T>>, T: Clone = ()> {
    strs:  PeekableStrs<'a, I, T>,
    first: bool,
    index: usize,
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> Lines<'a, I, T> {
    pub fn new(strs: I) -> Self {
        Self {
            strs:  PeekableStrs::new(strs),
            first: true,
            index: 0,
        }
    }
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> Iterator for Lines<'a, I, T> {
    type Item = Char<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chars) = self.strs.peek_mut() {
            if self.index == 0 {
                chars.skip_spaces();
            }

            if let Some(char) = chars.next() {
                let width = if char.is_newline() {
                    self.index = 0;
                    0
                } else {
                    let width = char.width();
                    self.index += width;
                    width
                };

                return Some(Char {
                    meta: chars.meta.clone(),
                    char,
                    width,
                });
            } else {
                self.strs.next();
                return self.next();
            }
        } else {
            return None;
        }
    }
}

// ======================================================================== //
// ======================================================================== //
// ======================================================================== //

#[derive(Clone, Debug)]
pub struct CroppedLines<'a, I: Iterator<Item = Str<'a, T>>, T: Clone = ()> {
    strs:  PeekableStrs<'a, I, T>,
    first: bool,
    index: usize,
    width: usize,
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> CroppedLines<'a, I, T> {
    pub fn new(strs: I, width: usize) -> Self {
        Self {
            strs: PeekableStrs::new(strs),
            first: true,
            index: 0,
            width,
        }
    }
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> Iterator for CroppedLines<'a, I, T> {
    type Item = Char<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chars) = self.strs.peek_mut() {
            if !self.first && self.index == 0 {
                chars.skip_spaces();
            }

            if let Some(char) = chars.peek() {
                if char.is_newline() {
                    self.index = 0;
                    self.first = false;
                    chars.next();

                    return Some(Char {
                        meta: chars.meta.clone(),
                        char,
                        width: 0,
                    });
                } else {
                    let width = char.width();

                    if self.index + width <= self.width {
                        self.index += width;
                        chars.next();

                        return Some(Char {
                            meta: chars.meta.clone(),
                            char,
                            width,
                        });
                    } else {
                        chars.skip_line();

                        return self.next();
                    }
                }
            } else {
                self.strs.next();

                return self.next();
            }
        } else {
            return None;
        }
    }
}

// ======================================================================== //
// ======================================================================== //
// ======================================================================== //

#[derive(Clone, Debug)]
pub struct CroppedLinesEllipsis<'a, I: Iterator<Item = Str<'a, T>>, T: Clone = ()> {
    strs:  PeekableStrs<'a, I, T>,
    first: bool,
    index: usize,
    width: usize,
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> CroppedLinesEllipsis<'a, I, T> {
    pub fn new(strs: I, width: usize) -> Self {
        Self {
            strs: PeekableStrs::new(strs),
            first: true,
            index: 0,
            width,
        }
    }
}

impl<'a, I: Iterator<Item = Str<'a, T>>, T: Clone> Iterator for CroppedLinesEllipsis<'a, I, T> {
    type Item = Char<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chars) = self.strs.peek_mut() {
            if !self.first && self.index == 0 {
                chars.skip_spaces();
            }

            if let Some(char) = chars.peek() {
                if char.is_newline() {
                    self.index = 0;
                    self.first = false;
                    chars.next();

                    return Some(Char::newline(chars.meta.clone()));
                } else {
                    let meta = chars.meta.clone();
                    let width = char.width();

                    if self.index + width > self.width {
                        self.index += 1;
                        self.strs.skip_line();

                        return Some(Char::ellipsis(meta));
                    } else if self.index + width == self.width {
                        chars.next();

                        if let Some(char_after) = self.strs.skip_spaces() {
                            if char_after.is_newline() {
                                self.index += width;

                                return Some(Char { meta, char, width });
                            } else {
                                self.index += 1;
                                self.strs.skip_line();

                                return Some(Char::ellipsis(meta));
                            }
                        } else {
                            self.index += width;

                            return Some(Char { meta, char, width });
                        }
                    } else {
                        self.index += width;
                        chars.next();

                        return Some(Char { meta, char, width });
                    }
                }
            } else {
                self.strs.next();

                return self.next();
            }
        } else {
            return None;
        }
    }
}
