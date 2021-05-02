use tender::style::*;

fn main() {
    let width = 151;
    let height = 40;

    dbg!(std::mem::size_of::<u8>());
    dbg!(std::mem::size_of::<Attributes>());
    dbg!(std::mem::size_of::<char>());
    dbg!(std::mem::size_of::<Rgb>());
    dbg!(std::mem::size_of::<Rgba>());
    dbg!(std::mem::size_of::<Cell<Rgb>>());
    dbg!(std::mem::size_of::<Option<Cell<Rgb>>>());
    dbg!(std::mem::size_of::<Cell<Rgba>>());
    dbg!(std::mem::size_of::<Option<Cell<Rgba>>>());
    dbg!(width * height * std::mem::size_of::<Cell<Rgba>>());
    dbg!(width * height * 20);
    dbg!(width * height * 16);
    dbg!(width * height * 12);

    /*
    struct C<C> {
        char:  char,
        fg:    C,
        bg:    C,
        attrs: Attrs,
    }
    dbg!(std::mem::size_of::<C<Rgb>>());
    dbg!(std::mem::size_of::<C<Rgba>>());
    */

    let u = 0b00001000;

    println!("\nSET (OR 1)");
    or(1, 8);
    or(9, 8);

    println!("\nIS_SET (AND 1 != 0)");
    and(1, 8);
    and(9, 8);

    println!("\nUNSET (AND !1)");
    and(1, !8);
    and(9, !8);
}

fn and(a: u8, b: u8) {
    println!(
        "\n  {:#010b} ({})\n\x1B[4m& {:#010b}\x1B[24m ({})\n  {:#010b} ({})",
        a,
        a,
        b,
        b,
        a & b,
        a & b,
    );
}

fn or(a: u8, b: u8) {
    println!(
        "\n  {:#010b} ({})\n\x1B[4m| {:#010b}\x1B[24m ({})\n  {:#010b} ({})",
        a,
        a,
        b,
        b,
        a | b,
        a | b,
    );
}
