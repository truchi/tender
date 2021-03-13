//! Colors ([`Foreground`](crate::style::Foreground),
//! [`Background`](crate::style::Background), [`Rgb`](crate::style::Rgb),
//! [`Rgba`](crate::style::Rgba), [`PreRgba`](crate::style::PreRgba)).

macro_rules! color {
    ($self:ident
        red       $red:block
        green     $green:block
        blue      $blue:block
        $(pre_red   $pre_red:block)?
        $(pre_green $pre_green:block)?
        $(pre_blue  $pre_blue:block)?
        alpha     $alpha:block
    ) => {
        fn red($self) -> u8 $red
        fn green($self) -> u8 $green
        fn blue($self) -> u8 $blue
        $(fn pre_red($self) -> u8 $pre_red)?
        $(fn pre_green($self) -> u8 $pre_green)?
        $(fn pre_blue($self) -> u8 $pre_blue)?
        fn alpha($self) -> u8 $alpha
    };
}

mod ground;
mod pre_rgba;
mod rgb;
mod rgba;

pub use ground::*;
pub use pre_rgba::*;
pub use rgb::*;
pub use rgba::*;

/// A trait for [`Foreground`](crate::style::Foreground),
/// [`Background`](crate::style::Background), [`Rgb`](crate::style::Rgb),
/// [`Rgba`](crate::style::Rgba), [`PreRgba`](crate::style::PreRgba).
///
/// Requires the convertions (with `C` in
/// [[`Rgb`](crate::style::Rgb), [`Rgba`](crate::style::Rgba),
/// [`PreRgba`](crate::style::PreRgba)]):
///   - `Self` <-> `C`
///   - `Self` <-> `Foreground<C>`
///   - `Self` <-> `Background<C>`
///
/// `From` impls on thoses types are hidden in the documentation.
pub trait Color: Copy {
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

    /// Returns true if `alpha` is `u8::MAX`.
    fn is_opaque(self) -> bool {
        self.alpha() == u8::MAX
    }

    /// Returns true if `alpha` is not `u8::MAX`.
    fn is_transparent(self) -> bool {
        !self.is_opaque()
    }

    /// Returns true if `alpha` is not `0`.
    fn is_visible(self) -> bool {
        !self.is_invisible()
    }

    /// Returns true if `alpha` is `0`.
    fn is_invisible(self) -> bool {
        self.alpha() == 0
    }

    /// Places `self` over `other`.
    fn over(self, other: Rgb) -> Rgb {
        let ratio = (u8::MAX as f64 - self.alpha() as f64) / u8::MAX as f64;
        let over = |a: u8, b: u8| a + (b as f64 * ratio) as u8;

        Rgb(
            over(self.pre_red(), other.red()),
            over(self.pre_green(), other.green()),
            over(self.pre_blue(), other.blue()),
        )
    }
}
