//! Attributes ([`Attributes`](crate::style::Attributes):
//! [`Weight`](crate::style::Weight), [`Slant`](crate::style::Slant),
//! [`Underline`](crate::style::Underline), [`Strike`](crate::style::Strike),
//! [`Overline`](crate::style::Overline), [`Invert`](crate::style::Invert),
//! [`Blink`](crate::style::Blink), [`Border`](crate::style::Border)).

/// `Attributes` ([`Weight`](crate::style::Weight),
/// [`Slant`](crate::style::Slant), [`Underline`](crate::style::Underline),
/// [`Strike`](crate::style::Strike), [`Overline`](crate::style::Overline),
/// [`Invert`](crate::style::Invert), [`Blink`](crate::style::Blink),
/// [`Border`](crate::style::Border)).
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Attributes {
    pub weight:    Option<Weight>,
    pub slant:     Option<Slant>,
    pub underline: Option<Underline>,
    pub strike:    Option<Strike>,
    pub overline:  Option<Overline>,
    pub invert:    Option<Invert>,
    pub blink:     Option<Blink>,
    pub border:    Option<Border>,
}

macro_rules! attr {
    ($(
        $(#[$attr_meta:meta])*
        $Attr:ident { $(
            $(#[$variant_meta:meta])*
            $Variant:ident
        )*;
            $(#[$default_meta:meta])*
            $Default:ident
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
    )* };
}

attr!(
    /// `Weight` (`Bold`, `Light`, `NoWeight`).
    Weight {
        ///
        Bold
        ///
        Light;
        ///
        NoWeight
    }
    /// `Slant` (`Italic`, `NoSlant`).
    Slant {
        ///
        Italic;
        ///
        NoSlant
    }
    /// `Underline` (`Underlined`, `NoUnderline`).
    Underline {
        ///
        Underlined;
        ///
        NoUnderline
    }
    /// `Strike` (`Striked`, `NoStrike`).
    Strike {
        ///
        Striked;
        ///
        NoStrike
    }
    /// `Overline` (`Overlined`, `NoOverline`).
    Overline {
        ///
        Overlined;
        ///
        NoOverline
    }
    /// `Invert` (`Inverted`, `NoInvert`).
    Invert {
        ///
        Inverted;
        ///
        NoInvert
    }
    /// `Blink` (`Slow`, `Fast`, `NoBlink`).
    Blink {
        ///
        Slow
        ///
        Fast;
        ///
        NoBlink
    }
    /// `Border` (`Circle`, `Frame`, `NoBorder`).
    Border {
        ///
        Circle
        ///
        Frame;
        ///
        NoBorder
    }
);
