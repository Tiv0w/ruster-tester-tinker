use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_the_number() {
    println!("{}", "Welcome to Guess the number!\n".underline());
    let secret_number: u8 = init_secret();

    println!("Input your guess:");
    loop {
        let guess: u8 = match handle_input() {
            (true, num) => num,
            (false, _) => continue,
        };
        if test_input(guess, &secret_number) {
            break;
        }
    }
}

fn init_secret() -> u8 {
    rand::thread_rng().gen_range(1, 101)
}

fn handle_input() -> (bool, u8) {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    return match guess.trim().parse() {
        Ok(num) => (true, num),
        Err(_) => {
            println!("Please enter a number between 1 and 100.");
            (false, 1)
        }
    };
}

fn test_input(guess: u8, secret: &u8) -> bool {
    match guess.cmp(secret) {
        Ordering::Less => {
            println!("It's more: {} {}", ">>".red(), guess);
            false
        }
        Ordering::Greater => {
            println!("It's less: {} {}", "<<".yellow(), guess);
            false
        }
        Ordering::Equal => {
            println!("\n{} is the correct number! ✅\nYou win! 🎉🎉", guess);
            true
        }
    }
}
