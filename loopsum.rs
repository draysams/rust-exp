fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let values = [8, 30, 1 , 3];
    let mut sum = 0;

    for i in values {
        sum = add(sum, i);
    }

    println!("sum = {}", sum);
}
