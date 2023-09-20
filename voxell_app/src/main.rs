use std::fmt;

fn lol() {
    println!("Hello, world!");
    println!("now i am become death, the destroy of worlds.");
    let variable: i8 = 32;
    println!("{} bit computers suck", variable);
    let first: &str = "tryharding";
    let last: &str = "having fun";
    println!(
        "i dont get why people be {0}, when im {0} im not {1}.",
        first, last
    );
    println!(
        "{ammo} bullets left. i think you should {action} now.",
        ammo = 3,
        action = "go back"
    );
    let number: i32 = 2147483646; // INT_MAX - 1 lmao
    let length: f32 = 1.5314513;
    println!(
        "Base representations of the same fucking number: {}",
        number
    );
    println!(
        "Base representations of the same fucking number: {:b}",
        number
    );
    println!(
        "Base representations of the same fucking number: {:o}",
        number
    );
    println!(
        "Base representations of the same fucking number: {:x}",
        number
    );
    println!(
        "Base representations of the same fucking number: {:X}",
        number
    );
    println!("Whoa, super long string!: \"{num:_>20}\"", num = number);
    println!("Whoa, super long gnirts!: \"{num:_<20}\"", num = number);

    println!(
        "Whoa, super variable-length string!: \"{num:_<length$}\"",
        num = number,
        length = 69 /* lol, nice */
    );

    println!(
        "Yo, this is {0} usage! You should {1} this!",
        "wrong", "dfghkfsdgsdf"
    );

    #[allow(dead_code)]
    struct Zorttirik(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    println!(
        "YOOO, KAN U DEBUG DIS? DEBUG DEEZ {:?}, HAHA, G{:?}TTEM",
        "NUTZ", 0
    );

    println!(
        "Just for the lulz, buddy. {}{}{}{}{}{}{}{}{}{}{}{}",
        number, number, number, length, number, length, number, first, last, first, last, last
    )
}

fn unlol() {
    println!();
    println!();
    println!();
    println!();
}

#[derive(Debug)]
struct ThePawn(f64);

#[derive(Debug)]
struct TheBishop(ThePawn);

#[derive(Debug)]
struct Car<'tf> {
    brand: &'tf str,
    price: i32,
}

struct Something(i32);

impl fmt::Display for Something {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
        /* note to self:
         * [Expression]; = expression, js: () => { 5; }
         * [Expression] = return value, js: () => 5
         */
    }
}

fn lol_the_second_coming() {
    println!("{:?} months in a year", 12);
    println!(
        "{subject:?} List:\n1. {0:?} {2:?}\n2. {1}",
        "Milk",
        "Chocolate Chips",
        3,
        subject = "Grocery"
    );
    println!("Why is {:#?} printable?", ThePawn(3.0));
    println!("I am just saying \"{:#?}\" amount of random words. What does a fractional word even mean? WTF?", TheBishop( ThePawn( 3.1415 ) ));

    let name: &str = "Lamborghini";
    let price: i32 = 9_500_293;
    let lambo: Car<'_> = Car { brand: name, price };

    println!("I bought a new car! She's so pretty!: {:#?}", lambo);
    println!(
        "Her name is {} and i bought her for {}$!",
        lambo.brand, lambo.price
    );

    println!("Fucking compiler errors... SMH.");

    // fmt time baby???
}

fn lol_the_final() {
    #[derive(Debug)]
    struct DoubleNumber(f64, f64);
    impl fmt::Display for DoubleNumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, ".[{} . {}].", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct PointTwoD {
        x: f64,
        y: f64,
    }

    impl fmt::Display for PointTwoD {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let something: DoubleNumber = DoubleNumber(0.183567158135, 0.96782658135);

    println!("Bro, check out this doublenumber i got! {:?}", something);
    println!("Bro, {}!", something);

    /// Complex number struct thing

    #[derive(Debug)]
    struct Complex {
        real: f32,
        imag: f32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let z: Complex = Complex {
        real: 0.0,
        imag: 1.0,
    };

    println!("Yoo, this number is in the mandelbrot set!: {}", z);
    println!("I dont know why but my computer is throwing a complex error! {:?}", z);
}

fn main() {
    lol();
    unlol();
    lol_the_second_coming();
    unlol();
    lol_the_final();
}
