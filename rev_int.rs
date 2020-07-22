fn reverse(mut n: usize) -> usize {
    let mut res = 0;

    while n != 0 {
        res = 10 * res + n % 10;
        n /= 10;
    }

    res
}

fn main() {
    let num = 1982;

    println!("{}", reverse(num));
}
