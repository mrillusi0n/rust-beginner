fn fizzbuzz(n: u32) -> String {
    let mut fin = String::new();
    let speaks = [(3, "Fizz"), (5, "Buzz")];

    for (num, speak) in &speaks {
        if n % num == 0 {
            fin += speak;
        }
    }

    if fin.is_empty() {
        fin = n.to_string();
    }

    fin
}

fn main() {
    for i in 0..16 {
        println!("{}", fizzbuzz(i));
    }
}
