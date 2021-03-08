use super::*;
use std::ops::{Deref, DerefMut};

macro_rules! from {
    ($Ground:ident($ground:ident) $($A:ident <-> $B:ident)*) => { $(
        // impl From<$A> for $Ground<$B> {
            // fn from(color: $A) -> Self {
                // Self(color.into())
            // }
        // }
        // impl From<$B> for $Ground<$A> {
            // fn from(color: $B) -> Self {
                // Self(color.into())
            // }
        // }
//
        // impl From<$Ground<$A>> for $B {
            // fn from($ground: $Ground<$A>) -> Self {
                // $ground.0.into()
            // }
        // }
        // impl From<$Ground<$B>> for $A {
            // fn from($ground: $Ground<$B>) -> Self {
                // $ground.0.into()
            // }
        // }

        impl From<$Ground<$A>> for $Ground<$B> {
            fn from($ground: $Ground<$A>) -> Self {
                $Ground($ground.0.into())
            }
        }
        impl From<$Ground<$B>> for $Ground<$A> {
            fn from($ground: $Ground<$B>) -> Self {
                $Ground($ground.0.into())
            }
        }
    )* };
}

macro_rules! ground {
    ($($(#[$meta:meta])* $Ground:ident($ground:ident))*) => { $(
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
        pub struct $Ground<T>(pub T);

        impl<T> Deref for $Ground<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        // TODO ???
        impl<T> $Ground<T> {
            pub fn color(self) -> T {
                self.0
            }

            pub fn into_color<U>(self) -> U
            where
                T: Into<U>
            {
                self.0.into()
            }

            pub fn from<U: Into<T>>(color: U) -> Self {
                Self(color.into())
            }

            pub fn into<U>(self) -> $Ground<U>
            where
                T: Into<U>
            {
                $Ground(self.0.into())
            }
        }

        impl<T> DerefMut for $Ground<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        from!($Ground($ground)
            Rgb      <->    Rgba
            Rgb      <-> PreRgba
            Rgba     <-> PreRgba
        );

        impl<T> From<T> for $Ground<T> {
            fn from(color: T) -> Self {
                Self(color)
            }
        }
    )* };
}

ground!(
    /// A `Foreground` wrapper for [`Color`](crate::style::Color)s.
    Foreground(foreground)
    /// A `Background` wrapper for [`Color`](crate::style::Color)s.
    Background(background)
);
