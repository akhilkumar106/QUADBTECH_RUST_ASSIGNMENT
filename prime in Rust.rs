// 10.Check if a number is prime in Rust
use std::io;

fn main() {
    println!("Enter a number to check if it's prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if num <= 1 {
        println!("{} is not a prime number.", num);
        return;
    }

    let mut is_prime = true;
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }

    if is_prime {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
