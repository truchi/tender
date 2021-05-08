//! Attributes
//! ([`Attributes`]: [`Weight`], [`Slant`], [`Underline`], [`Strike`]).

use super::*;
use std::fmt::{self, Debug, Display, Formatter};

// ------------------------------------------------------------------ //
//                                                                    //
// *************************** ATTRIBUTES *************************** //
//                                                                    //
// ------------------------------------------------------------------ //

/// `Attributes` ([`Weight`], [`Slant`], [`Underline`], [`Strike`]).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Attributes {
    pub weight:    Weight,
    pub slant:     Slant,
    pub underline: Underline,
    pub strike:    Strike,
}

impl From<()> for Attributes {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<(Weight, Slant, Underline, Strike)> for Attributes {
    fn from((weight, slant, underline, strike): (Weight, Slant, Underline, Strike)) -> Self {
        Self {
            weight,
            slant,
            underline,
            strike,
        }
    }
}

impl From<Attributes> for (Weight, Slant, Underline, Strike) {
    fn from(
        Attributes {
            weight,
            slant,
            underline,
            strike,
        }: Attributes,
    ) -> Self {
        (weight, slant, underline, strike)
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
    /// `Weight` (`Bold`, `Light`, `NoWeight`).
    Weight {
        ///
        Bold (1)
        ///
        Light (2);
        ///
        NoWeight (22)
    }
    /// `Slant` (`Italic`, `NoSlant`).
    Slant {
        ///
        Italic (3);
        ///
        NoSlant (23)
    }
    /// `Underline` (`Underlined`, `NoUnderline`).
    Underline {
        ///
        Underlined (4);
        ///
        NoUnderline (24)
    }
    /// `Strike` (`Striked`, `NoStrike`).
    Strike {
        ///
        Striked (9);
        ///
        NoStrike (29)
    }
);

// --------------------------------------------------------------------- //
//                                                                       //
// *************************** ATTRIBUTES U8 *************************** //
//                                                                       //
// --------------------------------------------------------------------- //

//         |- 00: NoWeight
//         |- 10: Dim
//         |- 11: Bold
//         | |- 0: NoStrike
//         | |- 1: Striked
//         | | |- 0: NoUnderline
// unused  | | |- 1: Underlined
//     |   | | | |- 0: NoItalic
// ____| __| | | |- 1: Italic
// 8 7 6 5 4 3 2 1
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct AttributesU8(u8);

impl AttributesU8 {
    const DIM_BOLD: u8 = 8;
    const SLANT: u8 = 1;
    const STRIKE: u8 = 4;
    const UNDERLINE: u8 = 2;
    const WEIGHT: u8 = 16;

    pub fn get_weight(self) -> Weight {
        if !self.is_set(Self::WEIGHT) {
            NoWeight
        } else if self.is_set(Self::DIM_BOLD) {
            Bold
        } else {
            Light
        }
    }

    pub fn set_weight(&mut self, weight: Weight) -> &mut Self {
        match weight {
            Bold => self.bold(),
            Light => self.light(),
            NoWeight => self.no_weight(),
        }
    }

    pub fn bold(&mut self) -> &mut Self {
        self.set(Self::WEIGHT).set(Self::DIM_BOLD)
    }

    pub fn light(&mut self) -> &mut Self {
        self.set(Self::WEIGHT).unset(Self::DIM_BOLD)
    }

    pub fn no_weight(&mut self) -> &mut Self {
        self.unset(Self::WEIGHT).unset(Self::DIM_BOLD)
    }

    pub fn get_slant(self) -> Slant {
        if self.is_set(Self::SLANT) {
            Italic
        } else {
            NoSlant
        }
    }

    pub fn set_slant(&mut self, slant: Slant) -> &mut Self {
        match slant {
            Italic => self.italic(),
            NoSlant => self.no_slant(),
        }
    }

    pub fn italic(&mut self) -> &mut Self {
        self.set(Self::SLANT)
    }

    pub fn no_slant(&mut self) -> &mut Self {
        self.unset(Self::SLANT)
    }

    pub fn get_underline(self) -> Underline {
        if self.is_set(Self::UNDERLINE) {
            Underlined
        } else {
            NoUnderline
        }
    }

    pub fn set_underline(&mut self, underline: Underline) -> &mut Self {
        match underline {
            Underlined => self.underlined(),
            NoUnderline => self.no_underline(),
        }
    }

    pub fn underlined(&mut self) -> &mut Self {
        self.set(Self::UNDERLINE)
    }

    pub fn no_underline(&mut self) -> &mut Self {
        self.unset(Self::UNDERLINE)
    }

    pub fn get_strike(self) -> Strike {
        if self.is_set(Self::STRIKE) {
            Striked
        } else {
            NoStrike
        }
    }

    pub fn set_strike(&mut self, strike: Strike) -> &mut Self {
        match strike {
            Striked => self.striked(),
            NoStrike => self.no_strike(),
        }
    }

    pub fn striked(&mut self) -> &mut Self {
        self.set(Self::STRIKE)
    }

    pub fn no_strike(&mut self) -> &mut Self {
        self.unset(Self::STRIKE)
    }

    fn set(&mut self, mask: u8) -> &mut Self {
        self.0 |= mask;
        self
    }

    fn unset(&mut self, mask: u8) -> &mut Self {
        self.0 &= !mask;
        self
    }

    fn is_set(&self, mask: u8) -> bool {
        (self.0 & mask) != 0
    }
}

impl From<Attributes> for AttributesU8 {
    fn from(
        Attributes {
            weight,
            slant,
            underline,
            strike,
        }: Attributes,
    ) -> Self {
        *Self::default()
            .set_weight(weight)
            .set_slant(slant)
            .set_underline(underline)
            .set_strike(strike)
    }
}

impl From<AttributesU8> for Attributes {
    fn from(attrs: AttributesU8) -> Self {
        Attributes {
            weight:    attrs.get_weight(),
            slant:     attrs.get_slant(),
            underline: attrs.get_underline(),
            strike:    attrs.get_strike(),
        }
    }
}

impl Display for AttributesU8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Attributes {
            weight,
            slant,
            underline,
            strike,
        } = (*self).into();

        write!(f, "{}{}{}{}", weight, slant, underline, strike)
    }
}

impl Display for Dedup<AttributesU8> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (
            Attributes {
                weight: previous_weight,
                slant: previous_slant,
                underline: previous_underline,
                strike: previous_strike,
            },
            Attributes {
                weight,
                slant,
                underline,
                strike,
            },
        ) = (self.0.into(), self.1.into());

        if weight != previous_weight {
            write!(f, "{}", weight)?;
        }
        if slant != previous_slant {
            write!(f, "{}", slant)?;
        }
        if underline != previous_underline {
            write!(f, "{}", underline)?;
        }
        if strike != previous_strike {
            write!(f, "{}", strike)?;
        }

        Ok(())
    }
}

impl Debug for AttributesU8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Attributes {
            weight,
            slant,
            underline,
            strike,
        } = (*self).into();

        f.debug_tuple("Attributes")
            .field(&weight)
            .field(&slant)
            .field(&underline)
            .field(&strike)
            .finish()
    }
}
