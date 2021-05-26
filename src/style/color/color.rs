use super::*;

/// A trait for colors.
pub trait Color:
    Copy
    + Default
    + PartialEq<Rgb>
    + Over<Rgb, Output = Rgb>
    + From<Rgb>
    + TryInto<Rgb>
    + PartialEq<Rgba>
    + Over<Rgba>
    + TryFrom<Rgba>
    + TryInto<Rgba>
    + PartialEq<PreRgba>
    + Over<PreRgba>
    + TryFrom<PreRgba>
    + Into<PreRgba>
{
    /// Returns the `alpha` component's value.
    fn get_alpha(self) -> u8;

    /// Returns the `alpha` component's value as `f64` ([0.0, 1.0]).
    fn get_alpha_f64(self) -> f64 {
        self.get_alpha() as f64 / u8::MAX as f64
    }

    /// Returns `1.0 / alpha_f64`.
    fn get_inv_alpha_f64(self) -> f64 {
        u8::MAX as f64 / self.get_alpha() as f64
    }

    /// Returns `1.0 - alpha_f64`.
    fn get_contr_alpha_f64(self) -> f64 {
        1.0 - self.get_alpha_f64()
    }

    /// Returns `true` if `alpha == u8::MAX`.
    fn is_opaque(self) -> bool {
        self.get_alpha() == u8::MAX
    }

    /// Returns `true` if `alpha != u8::MAX`.
    fn is_transparent(self) -> bool {
        self.get_alpha() != u8::MAX
    }

    /// Returns `true` if `alpha != 0`.
    fn is_visible(self) -> bool {
        self.get_alpha() != 0
    }

    /// Returns `true` if `alpha == 0`.
    fn is_invisible(self) -> bool {
        self.get_alpha() == 0
    }

    /// Converts `self` to `PreRgba`.
    fn pre(self) -> PreRgba {
        self.into()
    }
}
