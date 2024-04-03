//! This is a classic guessing game. The game will generate random number between 1 and 100.
//! The user must guess the number.

use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        let guess = ask();

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            },
        }
    }
}

/// Ask the user to guess the number and return it
fn ask() -> u32 {
    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return match guess.trim().parse::<u32>() {
        Ok(value) => value,
        Err(_) => ask()
    };
}