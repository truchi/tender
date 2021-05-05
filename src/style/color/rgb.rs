use super::*;
use std::fmt::{self, Display, Formatter};

/// Rgb color.
#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "2;{};{};{}", self.0, self.1, self.2)
    }
}

macro_rules! over {
    ($self:ident, $other:ident: $(
        $Self:ident: {
            WithAlpha $alpha:block
            $(From<$From:ident> $from:block)*
            $(PartialEq<$PartialEq:ident> $eq:block)*
            $(Over<$Bottom:ident, $Output:ident> $over:block)*
        }
    )*) => { $(
        impl WithAlpha for $Self { fn alpha($self) -> u8 $alpha }

        $(impl From<$From> for $Self {
            fn from($other: $From) -> $Self $from
        })*

        $(impl PartialEq<$PartialEq> for $Self {
            fn eq(&$self, $other: &$PartialEq) -> bool $eq
        })*

        $(over!(impl $self, $other: {
                $Self:
                    Over<     $Bottom, $Output> $over
                    Over<    &$Bottom, $Output> $self.over(*$other)
                    Over<&mut $Bottom, ()     > *$other = $self.over(*$other).into()
            } {
                &$Self:
                    Over<     $Bottom, $Output> (*$self).over( $other)
                    Over<    &$Bottom, $Output> (*$self).over(*$other)
                    Over<&mut $Bottom, ()     > (*$self).over( $other)
            } {
                &mut $Self:
                    Over<     $Bottom, ()     > *$self = (*$self).over($other).into()
                    Over<    &$Bottom, ()     > ($self).over(*$other)
            }
        );)*
    )* };
    (impl $self:ident, $bottom:ident: $(
        { $Top:ty: $(Over<$Bottom:ty, $Output:ty> $body:expr)* }
    )*) => { $( $(
        impl Over<$Bottom, $Output> for $Top {
            #[allow(unused_variables)]
            fn over($self, $bottom: $Bottom) -> $Output { $body }
        }
    )* )* }
}

over!(self, other:
    Rgb: {
        WithAlpha { u8::MAX }
        From<Rgba> { Rgb(other.0, other.1, other.2) }
        From<PreRgba> {
            if let Some(inv_alpha) = other.inv_alpha_f64() {
                Rgb(
                    (other.0 as f64 * inv_alpha).round() as _,
                    (other.1 as f64 * inv_alpha).round() as _,
                    (other.2 as f64 * inv_alpha).round() as _,
                )
            } else { Rgb(0, 0, 0) }
        }
        PartialEq<   Rgba> { Rgba::from(*self) == *other }
        PartialEq<PreRgba> { PreRgba::from(*self) == *other }
        Over<    Rgb, Rgb> { self }
        Over<   Rgba, Rgb> { self }
        Over<PreRgba, Rgb> { self }
    }
    Rgba: {
        WithAlpha { self.3 }
        From<   Rgb > { Rgba(other.0, other.1, other.2, u8::MAX) }
        From<PreRgba> {
            let Rgb(red, green, blue) = other.into();
            Rgba(red, green, blue, other.alpha())
        }
        PartialEq<   Rgb > { *self == Rgba::from(*other) }
        PartialEq<PreRgba> { PreRgba::from(*self) == *other }
        Over<    Rgb,    Rgb > { PreRgba::from(self).over(other) }
        Over<   Rgba, PreRgba> { PreRgba::from(self).over(other) }
        Over<PreRgba, PreRgba> { PreRgba::from(self).over(other) }
    }
    PreRgba: {
        WithAlpha { self.3 }
        From<Rgb> { PreRgba(other.0, other.1, other.2, u8::MAX) }
        From<Rgba> {
            let alpha = other.alpha_f64();
            PreRgba(
                (other.0 as f64 * alpha).round() as _,
                (other.1 as f64 * alpha).round() as _,
                (other.2 as f64 * alpha).round() as _,
                other.3,
            )
        }
        PartialEq<Rgb > { *self == PreRgba::from(*other) }
        PartialEq<Rgba> { *self == PreRgba::from(*other) }
        Over<Rgb, Rgb> {
            let contr_alpha = self.contr_alpha_f64();
            Rgb(
                self.0 + (other.0 as f64 * contr_alpha).round() as u8,
                self.1 + (other.1 as f64 * contr_alpha).round() as u8,
                self.2 + (other.2 as f64 * contr_alpha).round() as u8,
            )
        }
        Over<   Rgba, PreRgba> { self.over(PreRgba::from(other)) }
        Over<PreRgba, PreRgba> {
            let contr_alpha = self.contr_alpha_f64();
            PreRgba(
                self.0 + (other.0 as f64 * contr_alpha).round() as u8,
                self.1 + (other.1 as f64 * contr_alpha).round() as u8,
                self.2 + (other.2 as f64 * contr_alpha).round() as u8,
                self.3 + (other.3 as f64 * contr_alpha).round() as u8,
            )
        }
    }
);
