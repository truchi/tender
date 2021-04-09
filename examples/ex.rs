use tender::{canvas::*, style::*};

fn main() {
    println!("{:08b}", 0b00000001);
    println!("{:08b}", 0b00001111);
    println!("{:08b}", 9u8);
    dbg!(std::mem::size_of::<(u8, u8, u8, u8, u8, u8, u8, u8)>());
    dbg!(std::mem::size_of::<[bool; 8]>());
    dbg!(std::mem::size_of::<(
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool
    )>());
    // 8: 19 (27)
    // 8 16 24 32
    #[derive(Clone, Default)]
    struct Attributes2(u8, u8);

    #[derive(Clone, Default)]
    struct Styles2<C> {
        fg:         C,
        bg:         C,
        attributes: Attributes2,
    }

    #[derive(Clone, Default)]
    struct Cell2<C> {
        char:   char,
        styles: Styles2<C>,
    }

    #[derive(Clone, Default)]
    struct DamageCell2 {
        a: Cell2<Rgb>,
        b: Cell2<Rgb>,
    }

    dbg!(std::mem::size_of::<u8>());
    dbg!(std::mem::size_of::<char>());
    dbg!(std::mem::size_of::<Rgb>());
    dbg!(std::mem::size_of::<Rgba>());
    dbg!(std::mem::size_of::<Option<Rgba>>());
    dbg!(std::mem::size_of::<(Rgba, Rgba, char)>());
    println!();
    dbg!(std::mem::size_of::<Blink>());
    dbg!(std::mem::size_of::<Border>());
    dbg!(std::mem::size_of::<Invert>());
    dbg!(std::mem::size_of::<Overline>());
    dbg!(std::mem::size_of::<Slant>());
    dbg!(std::mem::size_of::<Strike>());
    dbg!(std::mem::size_of::<Underline>());
    dbg!(std::mem::size_of::<Weight>());
    println!();
    dbg!(std::mem::size_of::<Attributes>());
    dbg!(std::mem::size_of::<Styles<Rgb>>());
    dbg!(std::mem::size_of::<Styles<Rgba>>());
    dbg!(std::mem::size_of::<Cell<Rgb>>());
    dbg!(std::mem::size_of::<Cell<Rgba>>());
    dbg!(std::mem::size_of::<DamageCell>());
    println!();
    dbg!(std::mem::size_of::<Attributes2>());
    dbg!(std::mem::size_of::<Styles2<Rgb>>());
    dbg!(std::mem::size_of::<Styles2<Rgba>>());
    dbg!(std::mem::size_of::<Cell2<Rgb>>());
    dbg!(std::mem::size_of::<Cell2<Rgba>>());
    dbg!(std::mem::size_of::<DamageCell2>());

    fn a(w: usize, h: usize) {
        let len = w * h;

        println!();
        dbg!((w, h, len));

        dbg!(std::mem::size_of::<Cell<Rgba>>() * len);
        dbg!(std::mem::size_of::<DamageCell>() * len);

        let mut vec = Vec::<DamageCell>::with_capacity(len);
        vec.resize(len, Default::default());
        dbg!(std::mem::size_of_val(&vec[..]));
    }
    fn b(w: usize, h: usize) {
        let len = w * h;

        println!();
        dbg!((w, h, len));

        dbg!(std::mem::size_of::<Cell2<Rgba>>() * len);
        dbg!(std::mem::size_of::<DamageCell2>() * len);

        let mut vec = Vec::<DamageCell2>::with_capacity(len);
        vec.resize(len, Default::default());
        dbg!(std::mem::size_of_val(&vec[..]));
    }

    a(151, 40);
    b(151, 40);
    a(454, 127);
    b(454, 127);
}
