//! Attributes ([`Foreground`](crate::Foreground),
//! [`Background`](crate::Background), [`Weight`](crate::Weight),
//! [`Slant`](crate::Slant), [`Underline`](crate::Underline),
//! [`Strike`](crate::Strike), [`Overline`](crate::Overline),
//! [`Invert`](crate::Invert), [`Blink`](crate::Blink),
//! [`Border`](crate::Border)).

mod foreground;
pub use foreground::*;

mod background;
pub use background::*;

mod weight;
pub use weight::*;

mod slant;
pub use slant::*;

mod underline;
pub use underline::*;

mod strike;
pub use strike::*;

mod overline;
pub use overline::*;

mod invert;
pub use invert::*;

mod blink;
pub use blink::*;

mod border;
pub use border::*;
