fn main() {
    let add = |number1, number2| number1 + number2;

    let sum_range = |from, to| {
        let mut sum = 0;
        for n in from..to {
            sum = add(sum, n);
        }
        sum
    };

    println!("sum range = {}", sum_range(3, 10));
}
