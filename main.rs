fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let a = 38;
    let b = 4;
    let sum = add(a,b);

    println!("{} + {} = {}", a, b, sum);
}