fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }

    println!("{} = {} x {} + {}", a, b, a / b, a % b);
    gcd(b, a % b)
}

fn main() {
    let a = 510;
    let b = 92;

    println!("{}", gcd(a, b));
}
