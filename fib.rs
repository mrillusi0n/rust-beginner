fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    for i in 0..=10 {
        println!("{}", fib(i));
    }
}
