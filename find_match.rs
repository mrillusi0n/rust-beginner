fn find_match(txt: &str, start: usize) -> usize {
    let mut lvl = 0;

    for (i, ch) in txt[start..].chars().enumerate() {
        match ch {
            '(' => lvl += 1,
            ')' => lvl -= 1,
            _ => {}
        }

        if lvl == 0 {
            return i as usize;
        }
    }

    0
}

fn main() {
    println!("{}", find_match("(())", 1));
}
