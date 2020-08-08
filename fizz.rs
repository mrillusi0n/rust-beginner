fn fizz(n: u32) -> String {
    match n % 15 {
        0 => String::from("FizzBuzz"),
        5 | 10 => String::from("Buzz"),
        3 | 6 | 9 | 12 => String::from("Fizz"),
        _ => n.to_string(),
    }
}

fn main() {
    for i in 0..=100 {
        println!("{}", fizz(i));
    }
}
