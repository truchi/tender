use super::*;

macro_rules! ground_color {
    ($Ground:ident<$($Color:ident)*>) => { $(
        color!($Ground<$Color>, self
            red       { self.0.red() }
            green     { self.0.green() }
            blue      { self.0.blue() }
            pre_red   { self.0.pre_red() }
            pre_green { self.0.pre_green() }
            pre_blue  { self.0.pre_blue() }
            alpha     { self.0.alpha() }
        );
    )* };
}

macro_rules! convert {
    (From $(
        $FromColor:ident for $Ground:ident<$IntoColor:ident>
    )*) => { $(
        #[doc(hidden)]
        impl From<$FromColor> for $Ground<$IntoColor> {
            fn from(color: $FromColor) -> Self {
                Self(color.into())
            }
        }
    )* };
    (From $(
        $Ground:ident<$FromColor:ident> for $IntoColor:ident $(($into:ident))?
    )*) => { $(
        #[doc(hidden)]
        impl From<$Ground<$FromColor>> for $IntoColor {
            fn from(ground: $Ground<$FromColor>) -> Self {
                ground.0$(.$into())?
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

        ground_color!($Ground<Rgb Rgba PreRgba>);

        impl<T> From<T> for $Ground<T> {
            fn from(color: T) -> Self {
                Self(color)
            }
        }

        convert!(From
               Rgba for $Ground<   Rgb >
            PreRgba for $Ground<   Rgb >
               Rgb  for $Ground<   Rgba>
            PreRgba for $Ground<   Rgba>
               Rgb  for $Ground<PreRgba>
               Rgba for $Ground<PreRgba>
        );
        convert!(From
            $Ground<   Rgb > for    Rgb
            $Ground<   Rgba> for    Rgb  (into)
            $Ground<PreRgba> for    Rgb  (into)
            $Ground<   Rgba> for    Rgba
            $Ground<   Rgb > for    Rgba (into)
            $Ground<PreRgba> for    Rgba (into)
            $Ground<PreRgba> for PreRgba
            $Ground<   Rgb > for PreRgba (into)
            $Ground<   Rgba> for PreRgba (into)
        );
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

#[doc(hidden)]
impl<T> From<Foreground<T>> for Background<T> {
    fn from(foreground: Foreground<T>) -> Self {
        Background(foreground.0)
    }
}

#[doc(hidden)]
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
