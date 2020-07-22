fn is_vowel(c: char) -> bool {
    match c.to_lowercase().next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn piggify(word: String) -> String {
    let mut beginning = &word[1..];
    let mut first = word.chars().nth(0).unwrap();

    if is_vowel(first) {
        beginning = &word[..];
        first = 'h';
    }

    format!("{}-{}ay", beginning, first)
}

fn main() {
    let words = ["apple", "genie"];

    for word in &words {
        println!("{}", piggify(word.to_string()));
    }
}
