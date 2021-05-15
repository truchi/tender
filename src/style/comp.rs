use super::*;

/// A terminal `Cell`, composited.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Comp {
    pub(super) char:       char,
    pub(super) foreground: PreRgba,
    pub(super) background: PreRgba,
    pub(super) attributes: AttributesU8,
}

impl<Fg: Over<Bg>, Bg: Color> From<Cell<Fg, Bg>> for Comp
where
    <Fg as Over<Bg>>::Output: Color,
{
    fn from(cell: Cell<Fg, Bg>) -> Self {
        Self {
            char:       cell.char,
            foreground: cell.foreground.over(cell.background).into(),
            background: cell.background.into(),
            attributes: cell.attributes,
        }
    }
}

impl Over<Comp> for PreRgba {
    type Output = Comp;

    fn over(self, comp: Comp) -> Self::Output {
        Comp {
            char:       comp.char,
            foreground: self.over(comp.foreground).into(),
            background: self.over(comp.background).into(),
            attributes: comp.attributes,
        }
    }
}

impl Over<PreRgba> for Comp {
    type Output = Comp;

    fn over(self, color: PreRgba) -> Self::Output {
        Comp {
            char:       self.char,
            foreground: self.foreground.over(color).into(),
            background: self.background.over(color).into(),
            attributes: self.attributes,
        }
    }
}

impl Over<Comp> for Comp {
    type Output = Comp;

    fn over(self, bottom: Comp) -> Comp {
        if self.background.is_opaque() {
            debug_assert!(self.foreground.is_opaque());
            self
        } else if self.foreground == self.background {
            self.background.over(bottom)
        } else {
            self.over(bottom.background)
        }
    }
}

impl Over<&mut Comp> for &Comp {
    type Output = ();

    fn over(self, bottom: &mut Comp) {
        *bottom = (*self).over(*bottom);
    }
}

impl Over<Cell<Rgb, Rgb>> for Comp {
    type Output = Cell<Rgb, Rgb>;

    fn over(self, bottom: Cell<Rgb, Rgb>) -> Cell<Rgb, Rgb> {
        self.over(Self::from(bottom)).hard_into()
    }
}

impl Over<&mut Cell<Rgb, Rgb>> for &Comp {
    type Output = ();

    fn over(self, bottom: &mut Cell<Rgb, Rgb>) {
        *bottom = (*self).over(*bottom);
    }
}

/*
impl<Fg: Over<Bg>, Bg: Copy> From<Cell<Fg, Bg>> for Comp<Fg::Output, Bg> {
    fn from(cell: Cell<Fg, Bg>) -> Self {
        Self {
            char:       cell.char,
            foreground: cell.foreground.over(cell.background),
            background: cell.background,
            attributes: cell.attributes,
        }
    }
}

macro_rules! color_over_comp {
    ($($C:ident)*) => { $(
        impl<Fg, Bg> Over<Comp<Fg, Bg>> for $C
        where
            $C: Over<Fg> + Over<Bg>,
        {
            type Output = Comp<<$C as Over<Fg>>::Output, <$C as Over<Bg>>::Output>;

            fn over(self, comp: Comp<Fg, Bg>) -> Self::Output {
                Comp {
                    char:       comp.char,
                    foreground: self.over(comp.foreground),
                    background: self.over(comp.background),
                    attributes: comp.attributes,
                }
            }
        }
    )* };
}

macro_rules! comp_over_color {
    ($($C:ident)*) => { $(
        impl<Fg, Bg> Over<$C> for Comp<Fg, Bg>
        where
            Fg: Over<$C>,
            Bg: Over<$C>,
        {
            type Output = Comp<<Fg as Over<$C>>::Output, <Bg as Over<$C>>::Output>;

            fn over(self, color: $C) -> Self::Output {
                Comp {
                    char:       self.char,
                    foreground: self.foreground.over(color),
                    background: self.background.over(color),
                    attributes: self.attributes,
                }
            }
        }
    )* }
}

macro_rules! comp_rgb_rgb_over_comp {
    ($(Comp<$Fg:ty, $Bg:ty>)*) => { $(
        impl Over<Comp<$Fg, $Bg>> for Comp<Rgb, Rgb> {
            type Output = Comp<Rgb, Rgb>;

            fn over(self, _: Comp<$Fg, $Bg>) -> Self::Output {
                self
            }
        }
    )* };
}

macro_rules! comp_rgb_pre_rgba_over_comp {
    ($(Comp<$Fg:ty, $Bg:ty, Output = Comp<$NewFg:ty, $NewBg:ty>>)*) => { $(
        impl Over<Comp<$Fg, $Bg>> for Comp<Rgb, PreRgba> {
            type Output = Comp<$NewFg, $NewBg>;

            fn over(self, bottom: Comp<$Fg, $Bg>) -> Self::Output {
                self.over(bottom.background)
            }
        }
    )* };
}

macro_rules! comp_pre_rgba_pre_rgba_over_comp {
    ($(
        Comp<$Fg:ty, $Bg:ty, Output = Comp<$NewFg:ty, $NewBg:ty>>
        ($($cast1:ident)?) ($($cast2:ident)?)
    )*) => { $(
        impl Over<Comp<$Fg, $Bg>> for Comp<PreRgba, PreRgba> {
            type Output = Comp<$NewFg, $NewBg>;

            fn over(self, bottom: Comp<$Fg, $Bg>) -> Self::Output {
                if self.background.is_opaque() {
                    debug_assert!(self.foreground.is_opaque());
                    self$(.$cast1())?
                } else if self.foreground == self.background {
                    self.background.over(bottom)$(.$cast2())?
                } else {
                    self.over(bottom.background)
                }
            }
        }
    )* };
}

color_over_comp!(Rgb Rgba PreRgba);
comp_over_color!(Rgb Rgba PreRgba);
comp_rgb_rgb_over_comp!(
    Comp<   Rgb ,    Rgb >
    Comp<   Rgb ,    Rgba>
    Comp<PreRgba, PreRgba>
);
comp_rgb_pre_rgba_over_comp!(
    Comp<   Rgb ,    Rgb , Output = Comp<Rgb,    Rgb >>
    Comp<   Rgb ,    Rgba, Output = Comp<Rgb, PreRgba>>
    Comp<PreRgba, PreRgba, Output = Comp<Rgb, PreRgba>>
);
comp_pre_rgba_pre_rgba_over_comp!(
    Comp<   Rgb ,    Rgb , Output = Comp<   Rgb ,    Rgb >> (hard_cast) ()
    Comp<   Rgb ,    Rgba, Output = Comp<PreRgba, PreRgba>> (cast) (cast)
    Comp<PreRgba, PreRgba, Output = Comp<PreRgba, PreRgba>> () ()
);

// ```
//     Rgb     Rgb
//     Rgb  PreRgba
//  PreRgba PreRgba
//
// COMP OVER CELL
//    Rgb     Rgb  OVER    Rgb  Rgb (TOP)
//                         Rgba Rgb (TOP)
//                      PreRgba Rgb (TOP)
//
//    Rgb  PreRgba OVER    Rgb  Rgb
//                         Rgba Rgb
//                      PreRgba Rgb
//
// PreRgba PreRgba OVER    Rgb  Rgb
//                         Rgba Rgb
//                      PreRgba Rgb
// ```

impl Over<Cell<Rgb, Rgb>> for Comp<Rgb, Rgb> {
    type Output = Cell<Rgb, Rgb>;

    fn over(self, _: Cell<Rgb, Rgb>) -> Self::Output {
        self.into()
    }
}
impl Over<Cell<Rgba, Rgb>> for Comp<Rgb, Rgb> {
    type Output = Cell<Rgba, Rgb>;

    fn over(self, _: Cell<Rgba, Rgb>) -> Self::Output {
        Cell::from(self).cast::<Rgba, Rgb>()
    }
}
*/
