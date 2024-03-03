// 11.Merge two sorted arrays in Rust

use std::io;

fn merge_sorted_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut merged_array = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged_array.push(arr1[i]);
            i += 1;
        } else {
            merged_array.push(arr2[j]);
            j += 1;
        }
    }

    // Append remaining elements from arr1
    while i < arr1.len() {
        merged_array.push(arr1[i]);
        i += 1;
    }

    // Append remaining elements from arr2
    while j < arr2.len() {
        merged_array.push(arr2[j]);
        j += 1;
    }

    merged_array
}

fn main() {
    println!("Enter elements of the first sorted array (space-separated):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    let arr1: Vec<i32> = input1
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    println!("Enter elements of the second sorted array (space-separated):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let arr2: Vec<i32> = input2
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let merged_array = merge_sorted_arrays(arr1, arr2);

    println!("Merged array: {:?}", merged_array);
}
