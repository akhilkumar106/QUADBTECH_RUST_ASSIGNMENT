// 5. Given a sorted array of integers, implement a function that returns the median of the array.
use std::io;

fn find_median(nums: &mut [i32]) -> f64 {
    nums.sort();

    let len = nums.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (nums[mid_left] + nums[mid_right]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn main() {
    println!("Enter sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let mut nums = nums.into_boxed_slice(); // Convert to mutable slice

    let median = find_median(&mut nums);
    println!("The median of the array is: {}", median);
}
