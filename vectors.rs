fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    
    for num in &mut nums {
        *num *= 2;
    }

    for num in nums {
        println!("{}", num);
    }
}
