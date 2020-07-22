fn main() {
    let num = 7;

    println!("{}! = {}", num, factorial(num));
}

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
