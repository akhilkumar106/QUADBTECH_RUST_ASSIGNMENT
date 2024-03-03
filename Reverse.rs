// 9. Reverse a string in Rust

use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string to reverse:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let original_string = input.trim();
    let reversed_string = reverse_string(original_string);
    
    println!("Original: {}", original_string);
    println!("Reversed: {}", reversed_string);
}
