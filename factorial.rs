fn main() {
    let num = 7;

    println!("{}! = {}", num, factorial(num));
}

fn factorial(n: u64) -> u64 {
    if n == 1 {
        return n;
    }

    n * factorial(n - 1)
}
