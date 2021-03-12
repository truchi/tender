use super::*;

macro_rules! ground_color {
    ($Ground:ident) => {
        impl<T: Color> Color for $Ground<T> {
            color!(self
                red       { self.0.red() }
                green     { self.0.green() }
                blue      { self.0.blue() }
                pre_red   { self.0.pre_red() }
                pre_green { self.0.pre_green() }
                pre_blue  { self.0.pre_blue() }
                alpha     { self.0.alpha() }
            );
        }
    };
}

macro_rules! convert {
    ($Ground:ident $Other:ident $($A:ident <-> $B:ident)*) => {
        convert!($(impl From<$A> for $Ground<$B>)*);
        convert!($(impl From<$Ground<$A>> for $B)*);
        convert!($(impl From<$Ground<$A>> for $Ground<$B>)*);
        convert!($(impl From<$Other<$A>> for $Ground<$B>)*);
    };
    ($(impl From<$FromColor:ident> for $Ground:ident<$IntoColor:ident>)*) => { $(
        #[doc(hidden)]
        impl From<$FromColor> for $Ground<$IntoColor> {
            fn from(color: $FromColor) -> Self {
                Self(color.into())
            }
        }
    )* };
    ($(impl From<$Ground:ident<$FromColor:ident>> for $IntoColor:ident)*) => { $(
        #[doc(hidden)]
        impl From<$Ground<$FromColor>> for $IntoColor {
            fn from(ground: $Ground<$FromColor>) -> Self {
                ground.0.into()
            }
        }
    )* };
    ($(
        impl From<$FromGround:ident<$FromColor:ident>> for $IntoGround:ident<$IntoColor:ident>
    )*) => { $(
        #[doc(hidden)]
        impl From<$FromGround<$FromColor>> for $IntoGround<$IntoColor> {
            fn from(ground: $FromGround<$FromColor>) -> Self {
                Self(ground.0.into())
            }
        }
    )* };
}

macro_rules! ground {
    ($($(#[$meta:meta])* $Ground:ident ($other:ident: $Other:ident))*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
        pub struct $Ground<T>(pub T);

        ground_color!($Ground);

        impl<T> From<T> for $Ground<T> {
            fn from(color: T) -> Self {
                $Ground(color)
            }
        }

        impl<T> From<$Other<T>> for $Ground<T> {
            fn from($other: $Other<T>) -> Self {
                Self($other.0)
            }
        }

        convert!($Ground $Other
               Rgba <->    Rgb
            PreRgba <->    Rgb
               Rgb  <->    Rgba
            PreRgba <->    Rgba
               Rgb  <-> PreRgba
               Rgba <-> PreRgba
        );
    )* };
}

ground!(
    /// A `Foreground` wrapper for [`Color`](crate::style::Color)s.
    Foreground (background: Background)
    /// A `Background` wrapper for [`Color`](crate::style::Color)s.
    Background (foreground: Foreground)
);
