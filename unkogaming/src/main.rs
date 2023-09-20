use std::fmt::{self, Display, Formatter};

struct Village {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for Village {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ns = if self.lat >= 0.0 { 'N' } else { 'S' };
        let ew = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "[ Gotcha ADDRESS! {}: [{:.3}{}], [{:.3}{}] ]",
            self.name,
            self.lat.abs(),
            ns,
            self.lon.abs(),
            ew
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

fn main() {
    // stuff

    for address in [
        Village {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        Village {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        Village {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
        Village {
            name: "Momland",
            lat: 35.348343,
            lon: 3.000001,
        },
    ] {
        println!("And then he said {}", address);
    }

    for pixel in [
        Color {
            red: 34,
            green: 25,
            blue: 0,
        },
        Color {
            red: 255,
            green: 255,
            blue: 0,
        },
        Color {
            red: 0,
            green: 0,
            blue: 38,
        },
    ] {
        println!("Bruh, i got a dead {:?}, {}", pixel, pixel);
    }
}
