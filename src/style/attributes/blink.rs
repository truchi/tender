pub use Blink::*;

/// [`Blink`](crate::Blink) (`Slow`, `Fast`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Blink {
    Slow,
    Fast,
}
