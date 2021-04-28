/// A trait for colors.
pub trait WithAlpha: Copy + Default {
    /// Returns the `alpha` component's value.
    fn alpha(self) -> u8;

    /// Returns the `alpha` component's value as `f64` ([0.0, 1.0]).
    fn alpha_f64(self) -> f64 {
        self.alpha() as f64 / u8::MAX as f64
    }

    /// Returns `1.0 / alpha_f64`, or `None` if `alpha == 0`.
    fn inv_alpha_f64(self) -> Option<f64> {
        let alpha = self.alpha();

        if alpha == 0 {
            None
        } else {
            Some(u8::MAX as f64 / self.alpha() as f64)
        }
    }

    /// Returns `1.0 - alpha_f64`.
    fn contr_alpha_f64(self) -> f64 {
        1.0 - self.alpha_f64()
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
}
