//! Attributes ([`Attributes`](crate::style::Attributes):
//! [`Weight`](crate::style::Weight), [`Slant`](crate::style::Slant),
//! [`Underline`](crate::style::Underline), [`Strike`](crate::style::Strike),
//! [`Overline`](crate::style::Overline), [`Invert`](crate::style::Invert),
//! [`Blink`](crate::style::Blink), [`Border`](crate::style::Border)).

use std::fmt::{self, Display, Formatter};

/// `Attributes` ([`Weight`](crate::style::Weight),
/// [`Slant`](crate::style::Slant), [`Underline`](crate::style::Underline),
/// [`Strike`](crate::style::Strike), [`Overline`](crate::style::Overline),
/// [`Invert`](crate::style::Invert), [`Blink`](crate::style::Blink),
/// [`Border`](crate::style::Border)).
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Attributes {
    pub weight:    Weight,
    pub slant:     Slant,
    pub underline: Underline,
    pub strike:    Strike,
    pub overline:  Overline,
    pub invert:    Invert,
    pub blink:     Blink,
    pub border:    Border,
}

impl Display for Attributes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self {
            weight,
            slant,
            underline,
            strike,
            overline,
            invert,
            blink,
            border,
        } = self;

        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            weight, slant, underline, strike, overline, invert, blink, border,
        )
    }
}

macro_rules! attr {
    ($(
        $(#[$attr_meta:meta])*
        $Attr:ident { $(
            $(#[$variant_meta:meta])*
            $Variant:ident ($variant_csi:literal)
        )*;
            $(#[$default_meta:meta])*
            $Default:ident ($default_csi:literal)
        }
    )*) => { $(
        pub use $Attr::*;

        $(#[$attr_meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum $Attr {
            $(
                $(#[$variant_meta])*
                $Variant,
            )*
            $(#[$default_meta])*
            $Default
        }

        impl Default for $Attr {
            fn default() -> Self {
                $Attr::$Default
            }
        }

        impl Display for $Attr {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    $(Self::$Variant => write!(f, "\x1B[{}m", $variant_csi),)*
                    Self::$Default => write!(f, "\x1B[{}m", $default_csi),
                }
            }
        }
    )* };
}

attr!(
    /// `Weight` (`Bold`, `Light`, `ResetWeight`).
    Weight {
        ///
        Bold (1)
        ///
        Light (2);
        ///
        ResetWeight (22)
    }
    /// `Slant` (`Italic`, `ResetSlant`).
    Slant {
        ///
        Italic (3);
        ///
        ResetSlant (23)
    }
    /// `Underline` (`Underlined`, `ResetUnderline`).
    Underline {
        ///
        Underlined (4);
        ///
        ResetUnderline (24)
    }
    /// `Strike` (`Striked`, `ResetStrike`).
    Strike {
        ///
        Striked (9);
        ///
        ResetStrike (29)
    }
    /// `Overline` (`Overlined`, `ResetOverline`).
    Overline {
        ///
        Overlined (53);
        ///
        ResetOverline (55)
    }
    /// `Invert` (`Inverted`, `ResetInvert`).
    Invert {
        ///
        Inverted (7);
        ///
        ResetInvert (27)
    }
    /// `Blink` (`Slow`, `Fast`, `ResetBlink`).
    Blink {
        ///
        Slow (5)
        ///
        Fast (6);
        ///
        ResetBlink (25)
    }
    /// `Border` (`Circle`, `Frame`, `ResetBorder`).
    Border {
        ///
        Circle (52)
        ///
        Frame (51);
        ///
        ResetBorder (54)
    }
);
