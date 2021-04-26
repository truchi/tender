use std::fmt::{self, Display, Formatter};

pub use cell::*;
pub use colors::*;
pub use render::*;

pub const BLACK: Rgb = Rgb(0, 0, 0);
pub const WHITE: Rgb = Rgb(255, 255, 255);

/// Compositing trait
pub trait Over<Rhs = Self> {
    type Output;

    fn over(self, rhs: Rhs) -> Self::Output;
}

/// PreRgba and Rgb types
///
/// not much to see here
pub mod colors {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Default, Debug)]
    pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

    #[derive(Copy, Clone, PartialEq, Default, Debug)]
    pub struct Rgb(pub u8, pub u8, pub u8);

    impl From<Rgb> for PreRgba {
        fn from(Rgb(red, green, blue): Rgb) -> Self {
            Self(red, green, blue, u8::MAX)
        }
    }

    impl From<PreRgba> for Rgb {
        fn from(PreRgba(red, green, blue, alpha): PreRgba) -> Self {
            if alpha == 0 {
                Self(0, 0, 0)
            } else {
                let ratio = u8::MAX as f64 / alpha as f64;

                Self(
                    (ratio * red as f64).round() as _,
                    (ratio * green as f64).round() as _,
                    (ratio * blue as f64).round() as _,
                )
            }
        }
    }

    impl Over for PreRgba {
        type Output = Self;

        fn over(self, rhs: Self) -> Self {
            fn over(above: u8, below: u8, ratio: f64) -> u8 {
                above + (below as f64 * ratio).round() as u8
            }

            let ratio = 1.0 - (self.3 as f64 / u8::MAX as f64);
            Self(
                over(self.0, rhs.0, ratio),
                over(self.1, rhs.1, ratio),
                over(self.2, rhs.2, ratio),
                over(self.3, rhs.3, ratio),
            )
        }
    }

    impl Over<Rgb> for PreRgba {
        type Output = Rgb;

        fn over(self, rhs: Rgb) -> Rgb {
            self.over(PreRgba::from(rhs)).into()
        }
    }

    impl Display for Rgb {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "2;{};{};{}", self.0, self.1, self.2)
        }
    }

    impl PreRgba {
        fn top(self, bottom: Self) -> Self {
            // Over = Top + Bottom * (1 - AlphaTop)
            //
            // AlphaOver = AlphaTop + AlphaBottom * (1 - AlphaTop)
            // AlphaOver = AlphaTop + AlphaBottom - AlphaBottom * AlphaTop
            //
            // AlphaTop = AlphaOver - AlphaBottom * (1 - AlphaTop)
            // AlphaTop = AlphaOver - AlphaBottom + AlphaBottom * AlphaTop
            Default::default()
        }
    }
}

/// Cell type
///
/// IRL we have layers of cells
pub mod cell {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Cell {
        pub char: char,
        pub fg:   PreRgba,
        pub bg:   PreRgba,
    }

    impl Cell {
        pub fn new(char: char, fg: PreRgba, bg: PreRgba) -> Self {
            Self {
                char,
                fg: fg.over(bg), // We compose colors at construction!
                bg,
            }
        }
    }

    /// Compositing cells
    impl Over for Cell {
        type Output = Self;

        // T   top char
        // █   top bg
        // B   bottom char
        // █   bottom bg
        fn over(self, rhs: Self) -> Self {
            // remainder: color.3 is alpha channel

            if self.bg.3 == u8::MAX {
                // when top cell has opaque background,
                // all we see is the top cell
                //
                // T   top char
                // █   top bg       <- OPAQUE
                // B   bottom char  <- INVISIBLE
                // █   bottom bg    <- INVISIBLE
                self
            } else {
                // top cell has transparent background,
                // we see through
                //
                // T   top char     <- VISIBLE?
                // █   top bg       <- TRANSPARENT
                // B   bottom char  <- VISIBLE?
                // █   bottom bg    <- VISIBLE

                if self.fg != self.bg {
                    println!("top char visible");
                    // top char is visible,
                    // we merge both backgrounds
                    //
                    // T   top char     <- VISIBLE
                    // █   top bg       <- TRANSPARENT
                    // B   bottom char  <- INVISIBLE
                    // █   bottom bg    <- VISIBLE
                    Cell {
                        char: self.char,
                        fg:   self.fg.over(rhs.bg),
                        bg:   self.bg.over(rhs.bg),
                    }
                } else {
                    println!("top char invisible");
                    // top char is invisible,
                    // we see the bottom cell below a transparent color.
                    //
                    // T   top char     <- INVISIBLE
                    // █   top bg       <- TRANSPARENT
                    // B   bottom char  <- VISIBLE
                    // █   bottom bg    <- VISIBLE

                    // Now this is correct!
                    Cell {
                        char: rhs.char,
                        fg:   self.bg.over(rhs.fg),
                        bg:   self.bg.over(rhs.bg),
                    }
                }
            }
        }
    }
}

/// Render type: a rgb cell
///
/// (for debug and clarity)
pub mod render {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Render {
        pub char: char,
        pub fg:   Rgb,
        pub bg:   Rgb,
    }

    /// Rendering
    ///
    /// We force black background (for simplicity)
    impl From<Cell> for Render {
        fn from(cell: Cell) -> Self {
            // Correct!
            Self {
                char: cell.char,
                fg:   cell.fg.over(BLACK),
                bg:   cell.bg.over(BLACK),
            }
        }
    }

    impl Display for Render {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "\x1B[48;{}m", self.fg)?;
            write!(f, "\x1B[38;{}m", self.bg)?;
            write!(f, "{}", self.char)
        }
    }
}

fn main() {
    const TRANSPARENT: PreRgba = PreRgba(0, 0, 0, 0);
    const HALF_RED: PreRgba = PreRgba(127, 0, 0, 127);
    const HALF_GREEN: PreRgba = PreRgba(0, 127, 0, 127);
    const BLUE: PreRgba = PreRgba(0, 0, 255, 255);

    let top = Cell::new('T', TRANSPARENT, HALF_RED);
    let bottom = Cell::new('B', HALF_GREEN, BLUE);

    let composited = top.over(bottom);
    let rendered = Render::from(composited);

    let expected = Render {
        char: 'B',
        fg:   HALF_RED.over(HALF_GREEN).over(BLUE).into(),
        bg:   HALF_RED.over(BLUE).into(),
    };

    // println!("composited {:?}", composited);
    println!("rendered {:?}", rendered);
    println!("expected {:?}", expected);

    let top = Cell::new('T', HALF_RED, HALF_GREEN);
    let bottom = Cell::new('B', TRANSPARENT, BLUE);

    let composited = top.over(bottom);
    let rendered = Render::from(composited);

    let expected = Render {
        char: 'B',
        fg:   HALF_RED.over(HALF_GREEN).over(BLUE).into(),
        bg:   HALF_GREEN.over(BLUE).into(),
    };

    // println!("composited {:?}", composited);
    println!("rendered {:?}", rendered);
    println!("expected {:?}", expected);
}
