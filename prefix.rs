// 6. Implement a function that finds the longest common prefix of a given set of strings.

use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];

    for (index, character) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if index >= string.len() || string.chars().nth(index) != Some(character) {
                return first_string[..index].to_string();
            }
        }
    }

    first_string.to_string()
}

fn main() {
    println!("Enter strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<String> = input.split_whitespace().map(String::from).collect();

    let prefix = longest_common_prefix(&strings);
    if prefix.is_empty() {
        println!("No common prefix found.");
    } else {
        println!("The longest common prefix is: {}", prefix);
    }
}
