enum CharParity {
    Even,
    Odd,
    NaD,
}

fn char_parity(n: char) -> CharParity {
    let res = n as u8;

    match n {
        '0'..='9' => match res & 1 {
            0 => CharParity::Even,
            _ => CharParity::Odd,
        },
        _ => CharParity::NaD,
    }
}

fn main() {
    let nums_as_chars = ['%', '0', '7', 'o', '{', '2'];

    for num in nums_as_chars.iter() {
        match char_parity(*num) {
            CharParity::Even => println!("Even!"),
            CharParity::Odd => println!("Odd!"),
            CharParity::NaD => println!("Not a number :o"),
        }
    }
}
