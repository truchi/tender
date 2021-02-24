/// Moves cursor [`To`](crate::To) `column, row`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct To(pub u16, pub u16);

/// Moves cursor [`Right`](crate::Right) `n` columns.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Right(pub u16);

/// Moves cursor [`Left`](crate::Left) `n` columns.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Left(pub u16);

/// Moves cursor [`Down`](crate::Down) `n` lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Down(pub u16);
