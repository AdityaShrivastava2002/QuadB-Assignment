use std::io;

fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result: Option<usize> = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; // Move to the left to find the first occurrence
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the number to search:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let target: i32 = input.trim().parse().expect("Invalid number");

    if let Some(index) = first_occurrence_index(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}