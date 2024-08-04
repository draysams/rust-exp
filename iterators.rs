fn main() {
    let numbers = (3..10).inspect(|n| println!("n = {}", n));
    let mut sum = 0;
    let add = |n1, n2| n1 + n2;

    for n in numbers {
        sum = add(sum, n)
    }

    println!("sum = {}", sum)
}
