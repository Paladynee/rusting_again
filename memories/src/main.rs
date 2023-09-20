use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element: {}", slice[0]);
    println!("Slice length: {}", slice.len());
}

fn main() {
    // damn, initalizing fixed size arrays feel awkward after a lot of 2d game programming
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("Firsts!: {}, {}", xs[0], ys[0]);
    println!("Big ass array!: {}, {}", xs.len(), ys.len());

    println!(
        "Memory leak detected!! OMG!!!: {}, {}",
        mem::size_of_val(&xs),
        mem::size_of_val(&ys)
    );

    analyze_slice(&xs);
    analyze_slice(&xs[0..3]);

    // let empty: [u32; 0] = [];

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("Wtf is xval?: {}:  {}", i, xval),
            None => println!("Too far bruh {}", i),
        }
    }
}
