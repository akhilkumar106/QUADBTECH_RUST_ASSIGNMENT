// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number

use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            // Check if it's the first occurrence
            if mid == 0 || arr[mid - 1] < target {
                return Some(mid);
            } else {
                right = mid - 1;
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    println!("Enter the sorted array (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the number to search:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Invalid number");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}

