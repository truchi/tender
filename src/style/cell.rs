use super::*;
use std::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    ops::DerefMut,
};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Composited {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Straight {}

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<T, Fg, Bg = Fg> {
    char:       char,
    foreground: Color<Fg>,
    background: Color<Bg>,
    attributes: Attrs,
    phantom:    PhantomData<T>,
}

impl<Fg, Bg> Cell<Straight, Fg, Bg> {
    pub fn new(
        char: char,
        foreground: Fg,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self {
        Self {
            char,
            foreground: Color(foreground),
            background: Color(background),
            attributes: attributes.into().into(),
            phantom: PhantomData,
        }
    }
}

impl<Fg, Bg> Cell<Composited, Fg, Bg> {
    pub fn new<T>(
        char: char,
        foreground: T,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self
    where
        T: Over<Bg, Fg>,
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
