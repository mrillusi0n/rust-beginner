fn _search(k: i32, arr: &[i32], low: usize, high: usize) -> Option<usize> {
    let found: Option<usize>;
    let mid = (low + high) / 2;

    if low > high {
        return None;
    }

    if k == arr[mid] {
        found = Some(mid);
    } else if k > arr[mid] {
        found = _search(k, arr, mid + 1, high);
    } else {
        found = _search(k, arr, low, mid - 1);
    }

    found
}

fn search(k: i32, arr: &[i32], size: usize) -> Option<usize> {
    _search(k, arr, 0, size - 1)
}

fn main() {
    let nums = [3, 4, 5, 6, 7, 8];
    let num = 7;

    if let Some(i) = search(num, &nums, 6) {
        println!("{} was found at {} :D", num, i);
    } else {
        println!("{} was not found :[", num);
    }
}
