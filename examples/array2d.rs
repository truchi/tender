use tender::grid::*;

fn main() {
    let array2d = Array2D::<RowMajor, u8, [[u8; 3]; 3], [u8; 3]>::new_unchecked((3, 3), [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
    ]);

    println!("{:#?}", array2d);
}
