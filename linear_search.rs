fn search(arr: &[i32], k: i32) -> Option<usize> {
    for (i, num) in arr.iter().enumerate() {
        if *num == k {
            return Some(i);
        }
    }

    None
}

fn main() {
    let nums = [2, 1, 2, 4, 7, 0];
    let key = 8;

    if let Some(i) = search(&nums, key) {
        println!("{} was found at index {}.", key, i);
    } else {
        println!("{} was not found :[", key);
    }
}
