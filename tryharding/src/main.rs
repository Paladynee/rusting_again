use std::fmt;

struct FakeLinkedListLol(Vec<f64>);

impl fmt::Display for FakeLinkedListLol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "{{ ")?;

        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}: {}", i, v)?;
        }

        write!(f, " }}") // return value lol
    }
}

fn main() {
    let x: FakeLinkedListLol = FakeLinkedListLol(vec![
        1.1,
        4.4,
        5.3,
        6.2,
        6.1,
        35.3,
        4.51,
        4.64625,
        2.2465,
        43.24623532523,
    ]);
    println!("This is a fake linked list that i have generated, lol. its just a bunch of vectors.{}", x);
}
