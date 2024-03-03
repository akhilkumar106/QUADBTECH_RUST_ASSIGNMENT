// 3. Given a string of words, implement a function that returns the shortest word in the string.

use std::io;

fn shortest_word(string: &str) -> Option<&str> {
    string.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    let input_string = input_string.trim(); // Remove trailing newline

    match shortest_word(&input_string) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
