use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg> {
    char:       char,
    foreground: Fg,
    background: Bg,
    attributes: AttributesU8,
}

impl<Fg, Bg> Cell<Fg, Bg> {
    pub fn new(
        char: char,
        foreground: Fg,
        background: Bg,
        attributes: impl Into<Attributes>,
    ) -> Self {
        Self {
            char,
            foreground,
            background,
            attributes: attributes.into().into(),
        }
    }
}

impl<Fg: Color> Display for Cell<Fg, Rgb> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Foreground(self.foreground.over(self.background)),
            Background(self.background),
            self.attributes,
            self.char,
        )
    }
}

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
