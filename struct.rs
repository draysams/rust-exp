#[derive(Debug)]
struct Accumulator {
    sum: i32,
}

impl Accumulator {
    fn new(init: i32) -> Accumulator {
        Accumulator { sum: init }
    }
}

fn main() {
    let acc = Accumulator::new(42);

    println!("acc = {:?}", acc);
}
