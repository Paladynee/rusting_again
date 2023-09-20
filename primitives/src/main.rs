use std::fmt;

// fuckin primitives innit?
fn main() {
    let logic: bool = true;
    let money: i8 = -3; // debt lol
    let apples: u16 = 159; // cant have negative apples amirite
    let my_depression: f64 = 0.000000000001;

    let mut typeless = 12;
    typeless = 2147483648i64;

    let mut current_players = 8;
    current_players = 7; // Tim ragequit F

    // current_players = true; // we have true players playing on the server rn!

    let current_players = [1, 2, 3, 4]; // bruhzingo what get overwritten idiot lol

    let Jack_of_All_Trades = (
        1u8,
        1u16,
        1u32,
        1u64,
        1u128,
        1usize,
        -1i8,
        -1i16,
        -1i32,
        -1i64,
        -1i128,
        -1isize,
        1.01f32,
        1.02f64,
        'j',
        "jaj",
        true,
        (),
    );

    println!("1 + 0 = {}", 1i32 + 0);
    println!("1 + 2 = {}", 1i32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("10000 = {}", 1e4f64);

    println!("1 AND 0 = {}", true && false);
    println!("1 OR 0 = {}", true || false);
    println!("NOT 1 = {}", !true);

    println!("0101 AND 1010 = {:b}", 0b0101 & 0b1010);
    println!("0101 OR 1010 = {:b}", 0b0101 | 0b1010);
    println!("0101 XOR 1010 = {:b}", 0b0101 ^ 0b1010);

    println!("Evil bitshift hack!?!?  1 << 5 = {:b}", 1u32 << 5);
    println!("Good bit(un)shift hack!?!?  0x80 >> 5 = {:b}", 0x80u64 >> 5);

    println!("One mil{}lion", 1_000_000u32);

    lame();
}

fn lame() {
    let mat: Matrix = Matrix(1.5, 2.75, 1.25, 2.25);
    println!("My matrix looks like this:\n{}", mat);
    println!("Get transposed idiot lmao:\n{}", transpose(mat));
}

fn reverse(couple: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = couple;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({}, {})", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

fn transpose(mat: Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}
