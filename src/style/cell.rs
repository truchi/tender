use super::*;
use std::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    ops::DerefMut,
};

// ------------------------------------------------------------ //
//                                                              //
// *************************** CELL *************************** //
//                                                              //
// ------------------------------------------------------------ //

/*
/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg, Attrs> {
    char:       char,
    foreground: Color<Fg>,
    background: Color<Bg>,
    attributes: Attrs,
}

impl<Fg, Bg, Attrs> Cell<Fg, Bg, Attrs> {
    pub fn new(char: char, foreground: Fg, background: Bg, attributes: Attrs) -> Self {
        Self {
            char,
            foreground: Color(foreground),
            background: Color(background),
            attributes,
        }
    }
}

impl<Fg: Over<Rgb, Rgb> + Copy, T: Display> Display for Cell<Fg, Rgb, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Foreground(self.foreground.0.over(self.background.0)),
            Background(self.background.0),
            self.attributes,
            self.char,
        )
    }
}
*/

/*
impl<Fg, Bg> Cell<Composited, Fg, Bg> {
    pub fn new<C>(
        char: char,
        foreground: C,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self
    where
        C: Over<Bg, Fg>,
        Bg: Clone,
    {
        Self {
            char,
            foreground: Color(foreground.over(background.clone())),
            background: Color(background),
            attributes: attributes.into().into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Fg, Bg> Cell<T, Fg, Bg> {
    pub fn cast<NewFg, NewBg>(self) -> Cell<T, NewFg, NewBg>
    where
        Fg: Into<NewFg>,
        Bg: Into<NewBg>,
    {
        Cell {
            char:       self.char,
            foreground: Color(self.foreground.0.into()),
            background: Color(self.background.0.into()),
            attributes: self.attributes,
            phantom:    PhantomData,
        }
    }
}

impl<C: Over<Bg, Fg>, Fg, Bg: Clone> From<Cell<Straight, C, Bg>> for Cell<Composited, Fg, Bg> {
    fn from(cell: Cell<Straight, C, Bg>) -> Self {
        Self {
            char:       cell.char,
            foreground: cell.foreground.over(cell.background.clone()),
            background: cell.background,
            attributes: cell.attributes,
            phantom:    PhantomData,
        }
    }
}
*/

// ------------------------------------------------------------ //
//                                                              //
// *************************** COMP *************************** //
//                                                              //
// ------------------------------------------------------------ //

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
pub struct Comp<Fg, Bg, Attrs> {
    char:       char,
    foreground: Fg,
    background: Bg,
    attributes: Attrs,
}

impl<Fg, Bg, Attrs> Comp<Fg, Bg, Attrs> {
    pub fn new<C>(char: char, foreground: C, background: Bg, attributes: Attrs) -> Self
    where
        C: Over<Bg, Output = Fg>,
        Bg: Copy,
    {
        Self {
            char,
            foreground: foreground.over(background),
            background,
            attributes,
        }
    }

    pub fn cast<NewFg, NewBg, NewAttrs>(self) -> Comp<NewFg, NewBg, NewAttrs>
    where
        Fg: Into<NewFg>,
        Bg: Into<NewBg>,
        Attrs: Into<NewAttrs>,
    {
        Comp {
            char:       self.char,
            foreground: self.foreground.into(),
            background: self.background.into(),
            attributes: self.attributes.into(),
        }
    }
}

macro_rules! color_over_cell {
    ($($C:ident)*) => { $(
        impl<Fg, Bg, Attrs> Over<Comp<Fg, Bg, Attrs>> for $C
        where
            $C: Over<Fg> + Over<Bg>,
        {
            type Output = Comp<<$C as Over<Fg>>::Output, <$C as Over<Bg>>::Output, Attrs>;

            fn over(self, comp: Comp<Fg, Bg, Attrs>) -> Self::Output {
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

macro_rules! cell_over_color {
    ($($C:ident)*) => { $(
        impl<Fg, Bg, Attrs> Over<$C> for Comp<Fg, Bg, Attrs>
        where
            Fg: Over<$C>,
            Bg: Over<$C>,
        {
            type Output = Comp<<Fg as Over<$C>>::Output, <Bg as Over<$C>>::Output, Attrs>;

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

macro_rules! cell_over_cell {
    // ($self:ident $bottom:ident) => {};
    ($($C:ident)*) => { $(
        impl<TopFg, BottomFg, BottomBg, Attrs> Over<Comp<BottomFg, BottomBg, Attrs>>
            for Comp<TopFg, $C, Attrs>
        where
            TopFg: Over<BottomBg> + PartialEq<$C> + Into<PreRgba>,
            $C: Over<BottomFg> + Over<BottomBg>,
            <TopFg as Over<BottomBg>>::Output: Into<PreRgba>,
            <$C as Over<BottomFg>>::Output: Into<PreRgba>,
            <$C as Over<BottomBg>>::Output: Into<PreRgba>,
            // NOTE This bound shouldn't really be necessary! Why does the compiler need it?
            Self: Over<
                BottomBg,
                Output = Comp<<TopFg as Over<BottomBg>>::Output, <$C as Over<BottomBg>>::Output, Attrs>,
            >,
        {
            type Output = Comp<PreRgba, PreRgba, Attrs>;

            fn over(self, bottom: Comp<BottomFg, BottomBg, Attrs>) -> Self::Output {
                if self.background.is_opaque() {
                    self.cast()
                } else if self.foreground == self.background {
                    self.background.over(bottom).cast()
                } else {
                    self.over(bottom.background).cast()
                }
            }
        }
    )* };
}
// cell_over_cell!(Rgba PreRgba);

// impl<TopFg, BottomFg, BottomBg, Attrs> Over<Comp<BottomFg, BottomBg, Attrs>>
// for Comp<TopFg, Rgb, Attrs>
// {
// type Output = Comp<TopFg, Rgb, Attrs>;
//
// fn over(self, _: Comp<BottomFg, BottomBg, Attrs>) -> Self::Output {
// self
// }
// }

color_over_cell!(Rgb Rgba PreRgba);
cell_over_color!(Rgb Rgba PreRgba);

/*
impl<TopFg, BottomFg, BottomBg, Attrs> Over<Comp<BottomFg, BottomBg, Attrs>>
    for Comp<TopFg, Rgba, Attrs>
where
    TopFg: Over<BottomBg> + PartialEq<Rgba> + Into<PreRgba>,
    Rgba: Over<BottomFg> + Over<BottomBg>,
    <TopFg as Over<BottomBg>>::Output: Into<PreRgba>,
    <Rgba as Over<BottomFg>>::Output: Into<PreRgba>,
    <Rgba as Over<BottomBg>>::Output: Into<PreRgba>,
    // NOTE This bound shouldn't really be necessary! Why does the compiler need it?
    Self: Over<
        BottomBg,
        Output = Comp<<TopFg as Over<BottomBg>>::Output, <Rgba as Over<BottomBg>>::Output, Attrs>,
    >,
{
    type Output = Comp<PreRgba, PreRgba, Attrs>;

    fn over(self, bottom: Comp<BottomFg, BottomBg, Attrs>) -> Self::Output {
        if self.background.is_opaque() {
            self.cast()
        } else if self.foreground == self.background {
            self.background.over(bottom).cast()
        } else {
            self.over(bottom.background).cast()
        }
    }
}
*/

/*
// Color OVER Cell
impl<C, Fg, Bg, NewFg, NewBg> Over<Cell<Fg, Bg>, Cell<NewFg, NewBg>> for Color<C>
where
    C: Over<Fg, NewFg> + Over<Bg, NewBg> + Clone,
{
    fn over(self, cell: Cell<Fg, Bg>) -> Cell<NewFg, NewBg> {
        Cell {
            char:       cell.char,
            foreground: self.clone().over(cell.foreground),
            background: self.over(cell.background),
            attributes: cell.attributes,
        }
    }
}

// Cell OVER Color
impl<C, Fg, Bg, NewFg, NewBg> Over<Color<C>, Cell<NewFg, NewBg>> for Cell<Fg, Bg>
where
    C: Clone,
    Fg: Over<C, NewFg>,
    Bg: Over<C, NewBg>,
{
    fn over(self, color: Color<C>) -> Cell<NewFg, NewBg> {
        Cell {
            char:       self.char,
            foreground: self.foreground.over(color.clone()),
            background: self.background.over(color),
            attributes: self.attributes,
        }
    }
}

// Cell OVER Cell
impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    TopFg: Into<NewFg> + PartialEq<TopBg>,
    TopBg: Into<NewBg> + WithAlpha,
    Color<TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>>,
    Self: Over<Color<BottomBg>, Cell<NewFg, NewBg>>,
{
    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Cell<NewFg, NewBg> {
        if self.background.is_opaque() {
            self.cast()
        } else if self.foreground == self.background {
            self.background.over(bottom)
        } else {
            // full syntax or internal compilator error...
            <_ as Over<_, Cell<NewFg, NewBg>>>::over(self, bottom.background)
        }
    }
}

// Cell OVER &mut Cell
impl<TopFg, TopBg, BottomFg, BottomBg> Over<&mut Cell<BottomFg, BottomBg>, ()>
    for Cell<TopFg, TopBg>
where
    Cell<BottomFg, BottomBg>: Clone,
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, bottom: &mut Cell<BottomFg, BottomBg>) {
        *bottom = self.over(bottom.clone());
    }
}

// &Cell OVER &mut Cell
impl<TopFg, TopBg, BottomFg, BottomBg> Over<&mut Cell<BottomFg, BottomBg>, ()>
    for &Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Clone,
    Cell<BottomFg, BottomBg>: Clone,
    Cell<TopFg, TopBg>: Over<Cell<BottomFg, BottomBg>, Cell<BottomFg, BottomBg>>,
{
    fn over(self, bottom: &mut Cell<BottomFg, BottomBg>) {
        *bottom = self.clone().over(bottom.clone());
    }
}

impl Display for Cell<Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Foreground(self.foreground.0),
            Background(self.background.0),
            self.attributes,
            self.char,
        )
    }
}

impl Display for Dedup<Cell<Rgb>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Dedup(
                Foreground(self.0.foreground.0),
                Foreground(self.1.foreground.0)
            ),
            Dedup(
                Background(self.0.background.0),
                Background(self.1.background.0)
            ),
            Dedup(self.0.attributes, self.1.attributes),
            self.1.char,
        )
    }
}

// =================================================================================================

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct DamageCell {
    pub current:  Cell<Rgb>,
    pub previous: Cell<Rgb>,
}

impl DamageCell {
    pub fn new(cell: Cell<Rgb>) -> Self {
        Self {
            current:  cell,
            previous: cell,
        }
    }
}

pub trait WithDamage {
    fn cell(&mut self) -> &mut Cell<Rgb>;

    fn damage(&self) -> Option<Cell<Rgb>>;

    fn update(&mut self) -> Option<Cell<Rgb>>;
}

impl WithDamage for Cell<Rgb> {
    fn cell(&mut self) -> &mut Cell<Rgb> {
        self
    }

    fn damage(&self) -> Option<Cell<Rgb>> {
        Some(*self)
    }

    fn update(&mut self) -> Option<Cell<Rgb>> {
        Some(*self)
    }
}

impl<T: DerefMut<Target = DamageCell>> WithDamage for T {
    fn cell(&mut self) -> &mut Cell<Rgb> {
        &mut self.current
    }

    fn damage(&self) -> Option<Cell<Rgb>> {
        if self.current != self.previous {
            Some(self.current)
        } else {
            None
        }
    }

    fn update(&mut self) -> Option<Cell<Rgb>> {
        let damage = self.damage();
        self.previous = self.current;

        damage
    }
}

// Cell OVER impl WithDamage
impl<T, Fg, Bg> Over<T, ()> for Cell<Fg, Bg>
where
    T: WithDamage,
    Cell<Fg, Bg>: Over<Cell<Rgb, Rgb>, Cell<Rgb, Rgb>>,
{
    fn over(self, mut bottom: T) {
        *bottom.cell() = self.over(*bottom.cell());
    }
}

// &Cell OVER impl WithDamage
impl<T, Fg, Bg> Over<T, ()> for &Cell<Fg, Bg>
where
    T: WithDamage,
    Cell<Fg, Bg>: Clone,
    Cell<Fg, Bg>: Over<Cell<Rgb, Rgb>, Cell<Rgb, Rgb>>,
{
    fn over(self, mut bottom: T) {
        *bottom.cell() = self.clone().over(*bottom.cell());
    }
}
*/
