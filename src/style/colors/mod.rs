//! Colors ([`Foreground`](crate::style::Foreground),
//! [`Background`](crate::style::Background),
//! [`RgbTuple`](crate::style::RgbTuple), [`Rgb`](crate::style::Rgb),
//! [`Rgba`](crate::style::Rgba), [`PreRgba`](crate::style::PreRgba)).

mod ground;
mod pre_rgba;
mod rgb;
mod rgba;

pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;
