fn search(k: i32, arr: &[i32], low: usize, high: usize) -> bool {
    let mid = (low + high) / 2;

    if low > high {
        return false;
    }

    if k == arr[mid] {
        return true;
    } else if k > arr[mid] {
        return search(k, arr, mid + 1, high);
    } else {
        return search(k, arr, low, mid - 1);
    }
}

fn main() {
    let nums = [3, 4, 5, 6, 7, 8];
    let num = 7;

    if search(num, &nums, 0, 5) {
        println!("{} was found :D", num);
    } else {
        println!("{} was not found :[", num);
    }
}
