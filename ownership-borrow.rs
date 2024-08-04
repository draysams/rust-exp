struct Accumulator {
    sum: i32,
}

impl Accumulator {
    fn new(sum: i32) -> Self {
        Self { sum }
    }

    fn get(&self) -> i32 {
        self.sum
    }

    fn add(&mut self, increment: i32) {
        self.sum += increment
    }
}

fn main() {
    let mut acc = Accumulator::new(0);

    for n in 3..10 {
        acc.add(n);
        println!("n = {}", n);
    }

    println!("sum = {}", acc.get());
}
