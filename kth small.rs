// 7. Implement a function that returns the kth smallest element in a given array.

use std::io;

fn kth_smallest_element(nums: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= nums.len() {
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort();
        Some(sorted_nums[k - 1])
    } else {
        None
    }
}

fn main() {
    println!("Enter numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Invalid value of k");

    match kth_smallest_element(&nums, k) {
        Some(smallest) => println!("The {}rd smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
