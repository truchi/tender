pub use Blink::*;

/// [`Blink`](crate::Blink) (`Slow`, `Fast`, `NoBlink`).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Blink {
    Slow,
    Fast,
    NoBlink,
}

impl Default for Blink {
    fn default() -> Self {
        NoBlink
    }
}
