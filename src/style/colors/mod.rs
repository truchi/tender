//! Colors ([`Rgb`], [`Rgba`], [`PreRgba`]).

macro_rules! color {
    ($self:ident,  $color:ident: $T:ident
        from      $from:block // TODO remove
        red       $red:block
        green     $green:block
        blue      $blue:block
        $(pre_red   $pre_red:block)?
        $(pre_green $pre_green:block)?
        $(pre_blue  $pre_blue:block)?
        alpha     $alpha:block
        rgb       $rgb:block
        rgba      $rgba:block
        pre_rgba  $pre_rgba:block
    ) => {
        // fn from<$T: Color>($color: $T) -> Self $from
        fn red($self) -> u8 $red
        fn green($self) -> u8 $green
        fn blue($self) -> u8 $blue
        $(fn pre_red($self) -> u8 $pre_red)?
        $(fn pre_green($self) -> u8 $pre_green)?
        $(fn pre_blue($self) -> u8 $pre_blue)?
        fn alpha($self) -> u8 $alpha
        fn rgb($self) -> Rgb $rgb
        fn rgba($self) -> Rgba $rgba
        fn pre_rgba($self) -> PreRgba $pre_rgba
    };
}

macro_rules! from {
    ($self:ident: $Self:ident => $($from:ident: $From:ident)*) => { $(
        impl From<$From> for $Self {
            fn from($from: $From) -> Self {
                $from.$self()
            }
        }
    )* };
}

// mod ground;
mod pre_rgba;
mod rgb;
mod rgba;

// pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;

use super::*;

/// A trait for [`Rgb`], [`Rgba`], [`PreRgba`].
pub trait Color: Copy + Default {
    // /// Converts `color` into `Self`.
    // fn from<T: Color>(color: T) -> Self;

    /// Returns the `red` component's value.
    fn red(self) -> u8;

    /// Returns the `green` component's value.
    fn green(self) -> u8;

    /// Returns the `blue` component's value.
    fn blue(self) -> u8;

    /// Returns the `alpha` component's value.
    fn alpha(self) -> u8;

    /// Returns the `red` component's value,
    /// multiplied by the `alpha` component's value.
    fn pre_red(self) -> u8 {
        (self.alpha() as f64 / u8::MAX as f64 * self.red() as f64).round() as _
    }

    /// Returns the `green` component's value,
    /// multiplied by the `alpha` component's value.
    fn pre_green(self) -> u8 {
        (self.alpha() as f64 / u8::MAX as f64 * self.green() as f64).round() as _
    }

    /// Returns the `blue` component's value,
    /// multiplied by the `alpha` component's value.
    fn pre_blue(self) -> u8 {
        (self.alpha() as f64 / u8::MAX as f64 * self.blue() as f64).round() as _
    }

    /// Returns `true` if `alpha == u8::MAX`.
    fn is_opaque(self) -> bool {
        self.alpha() == u8::MAX
    }

    /// Returns `true` if `alpha != u8::MAX`.
    fn is_transparent(self) -> bool {
        self.alpha() != u8::MAX
    }

    /// Returns `true` if `alpha != 0`.
    fn is_visible(self) -> bool {
        self.alpha() != 0
    }

    /// Returns `true` if `alpha == 0`.
    fn is_invisible(self) -> bool {
        self.alpha() == 0
    }

    // /// Converts `self` into `T`.
    // fn into<T: Color>(self) -> T {
    //     T::from(self)
    // }

    /// Converts `self` into [`Rgb`].
    fn rgb(self) -> Rgb {
        Rgb(self.red(), self.green(), self.blue())
    }

    /// Converts `self` into [`Rgba`].
    fn rgba(self) -> Rgba {
        Rgba(self.red(), self.green(), self.blue(), self.alpha())
    }

    /// Converts `self` into [`PreRgba`].
    fn pre_rgba(self) -> PreRgba {
        PreRgba(
            self.pre_red(),
            self.pre_green(),
            self.pre_blue(),
            self.alpha(),
        )
    }

    /*
    /// Places `self` over `below`.
    fn over<U: Color>(self, below: impl Color) -> U {
        let above = self.pre_rgba();
        let below = below.pre_rgba();
        let ratio = 1.0 - (above.3 as f64 / u8::MAX as f64);

        fn over(above: u8, below: u8, ratio: f64) -> u8 {
            above + (below as f64 * ratio) as u8
        }

        Color::from(Rgba(
            over(above.0, below.0, ratio),
            over(above.1, below.1, ratio),
            over(above.2, below.2, ratio),
            over(above.3, below.3, ratio),
        ))
    }
    */
}
