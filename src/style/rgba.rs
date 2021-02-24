/// A rgba color for [`Foreground`](crate::Foreground) and
/// [`Background`](crate::Background).
///
/// To be used with [`Foreground`](crate::Foreground) and
/// [`Background`](crate::Background), as a [`Color`](crate::Color) does not
/// `Display` on its own.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgba(u8, u8, u8, u8);
