#![allow(dead_code)]

use core::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // lets try to fucking figure out what a struct or a tuple is lol

    let name = String::from("Robin Atkin Downes");
    let age = 99; // idfk
    let robin: Person = Person { name, age };
    println!("{:?}", robin);

    let point: Point = Point { x: 100.0, y: 200.0 };
    println!("Loc: ({}, {})", point.x, point.y);

    let point2: Point = Point {
        x: 2000.0,
        y: 400.0,
    };
    println!("Loc2: ({}, {})", point2.x, point2.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle: Rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: point2,
    };

    let _unit: Unit = Unit;
    let pair: Pair = Pair(1, 0.001);
    println!("Pairsfipsdfkjrqerk, {} {}", pair.0, pair.1);

    let Pair(theint, thedec) = pair;

    println!("Bruhing destructidfldsiferer {} {}", theint, thedec);
    println!("Area of {:?} is ->>> {}", rectangle, area(&rectangle))
}

fn area(rect: &Rectangle) -> f64 {
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;

    println!("{} {}", width, height);

    width * height
}
