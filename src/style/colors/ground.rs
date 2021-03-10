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
    ($Ground:ident From $($Color:ident)*) => { $(
        #[doc(hidden)]
        impl<T: Color> From<$Color> for $Ground<T> {
            fn from(color: $Color) -> Self {
                Self(color.into())
            }
        }
    )* };
    (($($Color:ident)*) From $Ground:ident) => { $(
        #[doc(hidden)]
        impl<T: Color> From<$Ground<T>> for $Color {
            fn from(ground: $Ground<T>) -> Self {
                ground.0.into()
            }
        }
    )* };
    (From $(
        $FromGround:ident<$FromColor:ident> for $IntoGround:ident<$IntoColor:ident>
    )*) => { $(
        #[doc(hidden)]
        impl From<$FromGround<$FromColor>> for $IntoGround<$IntoColor> {
            fn from(ground: $FromGround<$FromColor>) -> Self {
                $IntoGround(ground.0.into())
            }
        }
    )* };
}

macro_rules! ground {
    ($($(#[$meta:meta])* $Ground:ident)*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
        pub struct $Ground<T>(pub T);

        ground_color!($Ground);

        convert!($Ground From Rgb Rgba PreRgba);
        convert!((Rgb Rgba PreRgba) From $Ground);
        convert!(From
            $Ground<   Rgba> for $Ground<   Rgb >
            $Ground<PreRgba> for $Ground<   Rgb >
            $Ground<   Rgb > for $Ground<   Rgba>
            $Ground<PreRgba> for $Ground<   Rgba>
            $Ground<   Rgb > for $Ground<PreRgba>
            $Ground<   Rgba> for $Ground<PreRgba>
        );
    )* };
}

ground!(
    /// A `Foreground` wrapper for [`Color`](crate::style::Color)s.
    Foreground
    /// A `Background` wrapper for [`Color`](crate::style::Color)s.
    Background
);

impl<T> From<Foreground<T>> for Background<T> {
    fn from(foreground: Foreground<T>) -> Self {
        Background(foreground.0)
    }
}

impl<T> From<Background<T>> for Foreground<T> {
    fn from(background: Background<T>) -> Self {
        Foreground(background.0)
    }
}

convert!(From
    Foreground<   Rgba> for Background<   Rgb >
    Foreground<PreRgba> for Background<   Rgb >
    Foreground<   Rgb > for Background<   Rgba>
    Foreground<PreRgba> for Background<   Rgba>
    Foreground<   Rgb > for Background<PreRgba>
    Foreground<   Rgba> for Background<PreRgba>
    Background<   Rgba> for Foreground<   Rgb >
    Background<PreRgba> for Foreground<   Rgb >
    Background<   Rgb > for Foreground<   Rgba>
    Background<PreRgba> for Foreground<   Rgba>
    Background<   Rgb > for Foreground<PreRgba>
    Background<   Rgba> for Foreground<PreRgba>
);
