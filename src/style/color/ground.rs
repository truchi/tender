use super::*;

macro_rules! ground {
    ($($(#[$meta:meta])* $Ground:ident ($csi:literal))*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
        pub struct $Ground<T>(pub T);

        impl Display for CS<$Ground<Rgb>> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let Rgb(red, green, blue) = self.0.0;

                write!(f, concat!($csi, ";2;{};{};{}"), red, green, blue)
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
