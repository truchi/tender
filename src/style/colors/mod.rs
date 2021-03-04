//! Colors ([`Foreground`](crate::style::Foreground),
//! [`Background`](crate::style::Background),
//! [`RgbTuple`](crate::style::RgbTuple), [`Rgb`](crate::style::Rgb),
//! [`Rgba`](crate::style::Rgba), [`PreRgba`](crate::style::PreRgba)).

mod color;
mod ground;
mod pre_rgba;
mod rgb;
mod rgb_tuple;
mod rgba;

pub use color::*;
pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgb_tuple::*;
pub use rgba::*;
