use super::*;
use std::fmt::{self, Display, Formatter};

/// A terminal `Cell`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Cell<Fg, Bg> {
    pub char:       char,
    pub foreground: Fg,
    pub background: Bg,
    pub attributes: AttributesU8,
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

impl Display for Dedup<Cell<Rgb, Rgb>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            Dedup(Foreground(self.0.foreground), Foreground(self.1.foreground)),
            Dedup(Background(self.0.background), Background(self.1.background)),
            Dedup(self.0.attributes, self.1.attributes),
            self.1.char,
        )
    }
}

impl HardFrom<Comp> for Cell<Rgb, Rgb> {
    fn hard_from(comp: Comp) -> Self {
        debug_assert!(comp.foreground.is_opaque());
        debug_assert!(comp.background.is_opaque());

        Cell {
            char:       comp.char,
            foreground: comp.foreground.hard_into(),
            background: comp.background.hard_into(),
            attributes: comp.attributes,
        }
    }
}

impl<TopFg, TopBg, BottomFg, BottomBg> Over<Cell<BottomFg, BottomBg>> for Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Into<Comp>,
    Cell<BottomFg, BottomBg>: Into<Comp>,
{
    type Output = Comp;

    fn over(self, bottom: Cell<BottomFg, BottomBg>) -> Comp {
        self.into().over(bottom.into())
    }
}

impl<Fg, Bg> Over<&mut Cell<Rgb, Rgb>> for &Cell<Fg, Bg>
where
    Fg: Color,
    Bg: Color,
    Cell<Fg, Bg>: Into<Comp>,
{
    type Output = ();

    fn over(self, bottom: &mut Cell<Rgb, Rgb>) {
        (&(*self).into()).over(bottom);
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

// =================================================================================================

/*
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
