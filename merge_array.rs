use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged_array.push(arr1[i]);
            i += 1;
        } else {
            merged_array.push(arr2[j]);
            j += 1;
        }
    }

    merged_array.extend_from_slice(&arr1[i..]);
    merged_array.extend_from_slice(&arr2[j..]);

    merged_array
}

fn main() {
    println!("Enter elements of the first sorted array separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let arr1: Vec<i32> = input1.split_whitespace()
                                .map(|s| s.parse().expect("Invalid number"))
                                .collect();

    println!("Enter elements of the second sorted array separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let arr2: Vec<i32> = input2.split_whitespace()
                                .map(|s| s.parse().expect("Invalid number"))
                                .collect();

    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    
    println!("Merged sorted array: {:?}", merged_array);
}