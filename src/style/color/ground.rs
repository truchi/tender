use super::*;
use std::fmt::{self, Display, Formatter};

macro_rules! ground {
    ($($(#[$meta:meta])* $Ground:ident ($csi:literal))*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
        pub struct $Ground<T>(pub T);

        impl Display for $Ground<Rgb> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "\x1B[{};{}m", $csi, self.0)
            }
        }

        impl Display for Dedup<$Ground<Rgb>> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                if self.0 != self.1 {
                    self.1.fmt(f)
                }else {
                    Ok(())
                }
            }
        }
    )* };
}

ground!(
    /// A foreground wrapper for colors.
    Foreground (38)
    /// A background wrapper for colors.
    Background (48)
);
