use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the value of k:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let k: usize = input.trim().parse().expect("Invalid number");

    if let Some(kth_smallest) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k or array is too short");
    }
}