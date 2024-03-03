// 1. Implement a function that checks whether a given string is a palindrome or not.

use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Remove trailing newline

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
