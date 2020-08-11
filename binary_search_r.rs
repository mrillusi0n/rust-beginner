use std::cmp::Ordering;

fn _search(k: i32, arr: &[i32], low: usize, high: usize) -> Option<usize> {
    let mid = low + (high - low) / 2;

    if low > high {
        return None;
    }

    match k.cmp(&arr[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => _search(k, arr, mid + 1, high),
        Ordering::Less => _search(k, arr, low, mid - 1),
    }
}

fn search(k: i32, arr: &[i32], size: usize) -> Option<usize> {
    _search(k, arr, 0, size - 1)
}

fn main() {
    let nums = [3, 4, 5, 6, 7, 8];
    let num = 8;

    match search(num, &nums, 6) {
        Some(i) => println!("{} was found at index {} :D", num, i),
        None => println!("{} was not found :[", num),
    }
}
