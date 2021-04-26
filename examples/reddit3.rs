// 💬 Reddit thread:
// =================
// https://www.reddit.com/r/rust/comments/mvbn2g/compositing_colors/
//
// 📖 Some context:
// ================
//
// I am writing a lib for compositing and rendering in the terminal (for fun).
//
// We are working with cells and layers.
// - A cell is basically (char, fg: Rgba, bg: Rgba). Transparency yay!
// - A layer is a rectangle made of cells.
//
// We allow two operations: compositing and rendering.
// - Compositing is the process of merging a layer on top of another one, and
//   another one, and another...
// - Rendering is basically transforming a layer into something to output to the
//   screen (using CSIs for the terminal). It is somewhat convoluted in real
//   life, but for now we will consider it merely is converting transparent
//   layers into opaque layers using a default background color.
//
// It is important to note that compositing and rendering happen independently
// from each other. They are two separate phases of the pipeline. User can
// compose, compose again, do stuff, and finally render.
//
// I asked Reddit for help because I couldn't figure something out. I am not
// familiar with graphics in general. Heck, I don't even know what primary
// colors are! If you're noob like me read this:
// https://en.wikipedia.org/wiki/Alpha_compositing
//
// 🐈 What we want:
// ================
//
// So we want to merge cells. We have this:
// T   top char
// █   top bg
// B   bottom char
// █   bottom bg
//
// We want this:
// M   merged char
// █   merged bg
//
// We will use a simple algorithm:
// IF top's bg is opaque           // case 1
//     RETURN top
// ELSE IF top's char is visible   // case 2
//     RETURN top over bottom's bg
// ELSE                            // case 3
//     RETURN top's bg over bottom
//
// Case 1 is straightforward: we do not see through the top cell.
// Case 2 is also pretty easy: the background gets different.
// Case 3 got me headaches: we have to apply a color over a cell.
//
// Sounds easy? Well... if your first try is similar to mine, I am afraid it is
// not working as it should and you probably didn't even noticed it!
//
// 🦀 Let's do it!
// ===============
//
// This playground demonstrates my initial attempt and why it does not work. It
// also come with what I believe is the correct solution. I would like you to
// challenge it! We will test and explain each implementations.

pub use colors::*;

const TRANSPARENT: PreRgba = PreRgba(0, 0, 0, 0);
const HALF_RED: PreRgba = PreRgba(127, 0, 0, 127);
const HALF_GREEN: PreRgba = PreRgba(0, 127, 0, 127);
const BLUE: PreRgba = PreRgba(0, 0, 255, 255);
const BLACK: Rgb = Rgb(0, 0, 0);

/// Compositing trait (aka merging).
pub trait Over<Bottom = Self> {
    type Output;

    fn over(self, bottom: Bottom) -> Self::Output;
}

/// 🎨 Color types.
///
/// We have PreRgba (premultipled alpha RGBA) and Rgb, convertions from one
/// another, and of course PreRgba: Over<PreRgba> and PreRgba: Over<Rgb>.
///
/// Boring, skip.
pub mod colors {
    use super::*;

    #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct PreRgba(pub u8, pub u8, pub u8, pub u8);

    #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
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

        fn over(self, bottom: Self) -> Self {
            fn over(above: u8, below: u8, ratio: f64) -> u8 {
                above + (below as f64 * ratio).round() as u8
            }

            let ratio = 1.0 - (self.3 as f64 / u8::MAX as f64);
            Self(
                over(self.0, bottom.0, ratio),
                over(self.1, bottom.1, ratio),
                over(self.2, bottom.2, ratio),
                over(self.3, bottom.3, ratio),
            )
        }
    }

    impl Over<Rgb> for PreRgba {
        type Output = Rgb;

        fn over(self, bottom: Rgb) -> Rgb {
            self.over(PreRgba::from(bottom)).into()
        }
    }
}

/// 💥 The naive way.
///
/// In the naive way, we store fg and bg as is. Fg is then yet to be merged to
/// bg. It is what the user give us, not the "real screen" color.
/// Therefore when rendering a cell, we have 3 things to merge on top of the
/// others: the char fg, the cell bg and the default bg color.
pub mod bad {
    use super::*;

    /// Cell type.
    #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct Cell<T> {
        pub char: char,
        pub fg:   T,
        pub bg:   T,
    }

    impl Cell<PreRgba> {
        // Find the bug!
        pub fn new(char: char, fg: PreRgba, bg: PreRgba) -> Self {
            Self { char, fg, bg }
        }

        pub fn render(self, background: Rgb) -> Cell<Rgb> {
            let Self { char, fg, bg } = self;
            let bg = bg.over(background);
            let fg = fg.over(bg);

            // Sounds good!
            // *suspense music intensifies*
            Cell { char, fg, bg }
        }
    }

    /// Compositing cells.
    impl Over for Cell<PreRgba> {
        type Output = Self;

        fn over(self, bottom: Self) -> Self {
            // Case 1
            if self.bg.3 == u8::MAX {
                self // easy
            }
            // Case 2
            else if self.fg.3 != 0 {
                // Merge backgrounds
                Cell {
                    char: self.char,
                    fg:   self.fg,
                    bg:   self.bg.over(bottom.bg),
                }
            }
            // Case 3
            else {
                // Apply color on bottom.
                // What could go wrong?
                Cell {
                    char: bottom.char,
                    fg:   self.bg.over(bottom.fg),
                    bg:   self.bg.over(bottom.bg),
                }
            }
        }
    }

    // Let's see if this is working...
    pub fn test() {
        println!("BAD");

        // Case 1 and 2 work fine (I let you test that).
        // Our test case for case 3:
        // T   TRANSPARENT
        // █   HALF_RED
        // B   HALF_GREEN
        // █   BLUE

        let top = Cell::new('T', TRANSPARENT, HALF_RED);
        let bottom = Cell::new('B', HALF_GREEN, BLUE);

        // We expect:
        // B   HALF_RED over HALF_GREEN over BLUE
        // █   HALF_RED over BLUE
        //
        // Right?

        let expected = Cell::<Rgb> {
            char: 'B',
            fg:   HALF_RED.over(HALF_GREEN).over(BLUE).into(),
            bg:   HALF_RED.over(BLUE).into(),
        };

        let composited = top.over(bottom);
        let rendered = composited.render(BLACK /* whatever, bottom's bg is opaque */);

        assert!(rendered != expected); // 😕
        println!("rendered {:?}", rendered);
        println!("expected {:?}", expected);

        // Outputs:
        // rendered { 'B', Rgb(159, 64, 32), Rgb(127, 0, 128) }
        // expected { 'B', Rgb(127, 64, 64), Rgb(127, 0, 128) }
        //                       |       |
        //                       |       > too few blue!
        //                       > too much red!

        // ❓ So why is this not working?
        // ==============================
        //
        // In case 3, we apply the color of both fg and bg. When rendering we
        // merge again. So there is too much of the color in the char
        // (and too few of the background)!
        //
        // In real life, the color should overlay the same for each screen
        // pixels, regardless of being the char or the background!
        //
        // Another idea is to apply the color on the background only in case 3.
        // This is also incorrect because A OVER B != B OVER A...
    }
}

/// 👌 The (supposedly) correct implementation.
///
/// The idea is to merge the cell's fg to its bg beforehand. That way, the fg is
/// the "real screen" color (if terminals understood opacity). They are now
/// indepedent "channels". Thus, in the render phase, we simply merge both
/// "channels" on top of the default background.
pub mod good {
    use super::*;

    /// Cell type.
    #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
    pub struct Cell<T> {
        pub char: char,
        pub fg:   T,
        pub bg:   T,
    }

    impl Cell<PreRgba> {
        pub fn new(char: char, fg: PreRgba, bg: PreRgba) -> Self {
            Self {
                char,
                fg: fg.over(bg), // Look, im clever!
                bg,
            }
        }

        pub fn render(self, background: Rgb) -> Cell<Rgb> {
            let Self { char, fg, bg } = self;

            Cell {
                char,
                fg: fg.over(background),
                bg: bg.over(background),
            }
        }
    }

    /// Compositing cells.
    impl Over for Cell<PreRgba> {
        type Output = Self;

        fn over(self, bottom: Self) -> Self {
            // Case 1
            if self.bg.3 == u8::MAX {
                self // easy
            }
            // Case 2
            // (This is how we test for fg' visibility now)
            else if self.fg != self.bg {
                // Merge backgrounds
                Cell {
                    char: self.char,
                    fg:   self.fg.over(bottom.bg), // We also apply on fg now
                    bg:   self.bg.over(bottom.bg),
                }
            }
            // Case 3
            else {
                // Apply color on bottom.
                // Note that this is still the same, but now it works!
                Cell {
                    char: bottom.char,
                    fg:   self.bg.over(bottom.fg),
                    bg:   self.bg.over(bottom.bg),
                }
            }
        }
    }

    // It works!
    pub fn test() {
        println!("GOOD");

        // Case 1 and 2 work fine (I let you test that).
        // Our test case for case 3 is the same as above:
        // T   TRANSPARENT
        // █   HALF_RED
        // B   HALF_GREEN
        // █   BLUE

        let top = Cell::new('T', TRANSPARENT, HALF_RED);
        let bottom = Cell::new('B', HALF_GREEN, BLUE);

        // We expect:
        // B   HALF_RED over HALF_GREEN over BLUE
        // █   HALF_RED over BLUE
        //
        // Still ok?

        let expected = Cell::<Rgb> {
            char: 'B',
            fg:   HALF_RED.over(HALF_GREEN).over(BLUE).into(),
            bg:   HALF_RED.over(BLUE).into(),
        };

        let composited = top.over(bottom);
        let rendered = composited.render(BLACK /* whatever, bottom's bg is opaque */);

        assert!(rendered == expected); // 😀
        println!("rendered {:?}", rendered);
        println!("expected {:?}", expected);

        // Outputs:
        // rendered { 'B', Rgb(127, 64, 64), Rgb(127, 0, 128) }
        // expected { 'B', Rgb(127, 64, 64), Rgb(127, 0, 128) }
        //                     -----------
        //                        same!

        // ❗ Great!
        // =========
        //
        // I hope I made things clear!
        // I am waiting for you in the above Reddit link to tell me I am right
        // (or wrong!).
    }
}

pub fn main() {
    bad::test();
    good::test();
    test();
}

fn test() {
    use good::*;

    println!("BONUS");

    // We cannot leave without testing case 2!
    // T   HALF_RED
    // █   HALF_GREEN
    // B   TRANSPARENT
    // █   BLUE

    let top = Cell::new('T', HALF_RED, HALF_GREEN);
    let bottom = Cell::new('B', TRANSPARENT, BLUE);

    // We expect:
    // T   HALF_RED over HALF_GREEN over BLUE
    // █   HALF_GREEN over BLUE
    //
    // Right?

    let expected = Cell::<Rgb> {
        char: 'T',
        fg:   HALF_RED.over(HALF_GREEN).over(BLUE).into(),
        bg:   HALF_GREEN.over(BLUE).into(),
    };

    let composited = top.over(bottom);
    let rendered = composited.render(BLACK /* whatever, bottom's bg is opaque */);

    assert!(rendered == expected); // 😀
    println!("rendered {:?}", rendered);
    println!("expected {:?}", expected);

    // Outputs:
    // rendered { 'T', Rgb(127, 64, 64), Rgb(0, 127, 128) }
    // expected { 'T', Rgb(127, 64, 64), Rgb(0, 127, 128) }
}
