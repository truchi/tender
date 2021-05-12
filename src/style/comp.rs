use super::*;

/// A terminal `Cell`, composited.
///
/// Composited cells, 5 possibilities:
/// ```
///     Rgb     Rgb
///     Rgb  PreRgba
///  PreRgba PreRgba
/// (   Rgb     Rgba)
/// (PreRgba    Rgba)
///
/// OVER
///    Rgb     Rgb  OVER    Rgb     Rgb  => Rgb Rgb (TOP)
///                         Rgb  PreRgba => Rgb Rgb (TOP)
///                      PreRgba PreRgba => Rgb Rgb (TOP)
///
///    Rgb  PreRgba OVER    Rgb     Rgb  => Rgb    Rgb
///                         Rgb  PreRgba => Rgb PreRgba
///                      PreRgba PreRgba => Rgb PreRgba
///
/// PreRgba PreRgba OVER    Rgb     Rgb  =>    Rgb     Rgb
///                         Rgb  PreRgba => PreRgba PreRgba
///                      PreRgba PreRgba => PreRgba PreRgba
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Comp<Fg, Bg> {
    pub(super) char:       char,
    pub(super) foreground: Fg,
    pub(super) background: Bg,
    pub(super) attributes: AttributesU8,
}

impl<Fg, Bg> Comp<Fg, Bg> {
    pub fn new<C>(
        char: char,
        foreground: C,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self
    where
        C: Over<Bg, Output = Fg>,
        Bg: Copy,
    {
        Self {
            char,
            foreground: foreground.over(background),
            background,
            attributes: attributes.into().into(),
        }
    }

    fn cast<NewFg, NewBg>(self) -> Comp<NewFg, NewBg>
    where
        Fg: Into<NewFg>,
        Bg: Into<NewBg>,
    {
        Comp {
            char:       self.char,
            foreground: self.foreground.into(),
            background: self.background.into(),
            attributes: self.attributes,
        }
    }

    fn hard_cast<NewFg, NewBg>(self) -> Comp<NewFg, NewBg>
    where
        Fg: HardInto<NewFg>,
        Bg: HardInto<NewBg>,
    {
        Comp {
            char:       self.char,
            foreground: self.foreground.hard_into(),
            background: self.background.hard_into(),
            attributes: self.attributes,
        }
    }
}

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
