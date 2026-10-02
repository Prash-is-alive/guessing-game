use rand::prelude::*;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the numbers");
    println!("Please enter your guess:");

    // Generate the secret number
    let secret_num = rand::rng().random_range(1..=100);

    loop {
        // Take input from the user
        let mut guess = String::new(); // creates an mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // Convert the user input string into a integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too less!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

        println!("You guessed: {guess}");
    }
}
