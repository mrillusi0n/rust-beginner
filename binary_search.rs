use std::cmp::Ordering;

fn search(k: i32, arr: &[i32], size: usize) -> Option<usize> {
    let mut left = 0;
    let mut right = size - 1;
    let mut mid: usize;

    while left <= right {
        mid = (left + right) / 2;

        match k.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => right = mid - 1,
            Ordering::Greater => left = mid + 1,
        }
    }

    None
}

fn main() {
    let nums = [3, 4, 5, 6, 7, 8];
    let num = 5;

    if let Some(i) = search(num, &nums, 6) {
        println!("{} was found at {} :D", num, i);
    } else {
        println!("{} was not found :[", num);
    }
}
