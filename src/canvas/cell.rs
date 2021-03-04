use crate::canvas::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg = Fg> {
    pub char:   char,
    pub styles: Styles<Fg, Bg>,
}

impl Cell<PreRgba, Rgb> {
    pub fn flatten(self) -> Cell<Rgb> {
        Cell {
            char:   self.char,
            styles: self.styles.flatten(),
        }
    }
}

impl Cell<PreRgba> {
    /// Places `self` over `other`.
    pub fn over(self, other: Cell<Rgb>) -> Cell<Rgb> {
        let foreground = self.styles.foreground.0;
        let background = self.styles.background.0;

        // When self has opaque background, other is invisible
        if background.is_opaque() {
            // self.flatten()
            // Blend foreground over background
            let background = background.into();
            let foreground = foreground.over(background);

            Cell {
                char:   self.char,
                styles: Styles {
                    foreground: Foreground(foreground),
                    background: Background(background),
                    ..self.styles.into()
                },
            }
        }
        // Here we blend
        else {
            if foreground.alpha == 0 {
            } else {
            }

            other
        }

        // // When self has opaque background, other is invisible
        // if self.style.background.0.alpha == u8::MAX {
        //     self
        // } else {
        //     let background =
        // self.style.background.0.over(other.style.background.0);

        //     let cell = if self.style.foreground.0.alpha == u8::MAX {
        //         self
        //     } else {
        //         let foreground =
        // self.style.background.0.over(other.style.foreground.0);
        //         other
        //     };

        //     other
        // }
    }
}

macro_rules! from {
    ($($From:ident for $For:ident)*) => { $(
        impl From<Cell<$From>> for Cell<$For> {
            fn from(cell: Cell<$From>) -> Self {
                Self {
                    char:  cell.char,
                    styles: cell.styles.into(),
                }
            }
        }
    )* };
}

from!(
    Rgba    for Rgb
    PreRgba for Rgb
    Rgb     for Rgba
    PreRgba for Rgba
    Rgb     for PreRgba
    Rgba    for PreRgba
);

// #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
// pub struct DamageCell {
// pub new: Cell,
// pub old: Option<Cell>,
// }
//
// impl From<Cell> for DamageCell {
// fn from(new: Cell) -> Self {
// DamageCell { new, old: None }
// }
// }
