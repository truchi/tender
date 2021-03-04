use super::*;

pub trait Color:
    Copy
    + From<RgbTuple>
    + From<Rgb>
    + From<Rgba>
    + From<PreRgba>
    + Into<RgbTuple>
    + Into<Rgb>
    + Into<Rgba>
    + Into<PreRgba>
{
    fn red(self) -> u8 {
        Into::<Rgb>::into(self).red
    }

    fn green(self) -> u8 {
        Into::<Rgb>::into(self).green
    }

    fn blue(self) -> u8 {
        Into::<Rgb>::into(self).blue
    }

    fn pre_red(self) -> u8 {
        Into::<PreRgba>::into(self).red
    }

    fn pre_green(self) -> u8 {
        Into::<PreRgba>::into(self).green
    }

    fn pre_blue(self) -> u8 {
        Into::<PreRgba>::into(self).blue
    }

    fn alpha(self) -> u8 {
        Into::<Rgba>::into(self).alpha
    }

    fn is_opaque(self) -> bool {
        self.alpha() == u8::MAX
    }

    fn is_transparent(self) -> bool {
        !self.is_opaque()
    }
}
