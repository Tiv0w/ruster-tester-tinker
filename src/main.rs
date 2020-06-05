mod fahrenheit_celsius;
mod fibonacci;
mod guess_the_number;
use colored::*;
use std::io;

fn main() {
    println!("{}", "\nWelcome to my Toolbox!\n".purple().bold());
    println!("Select the tool you want:");
    println!("1) Fibonacci");
    println!("2) Guess The Number");
    println!("3) Fahrenheit - Celsius converter");
    println!("---");
    println!("Choice:");

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read your choice.");
    println!("---\n");

    let choice: u8 = choice
        .trim()
        .parse()
        .expect("There was a problem with the choice selection.");
    match choice {
        1 => {
            for num in 1..20 {
                println!("{}", fibonacci::pretty_fibonacci(num));
            }
        }
        2 => guess_the_number::guess_the_number(),
        3 => fahrenheit_celsius::start_interactive_converter(),
        _ => println!("The tool selected is incorrect."),
    }
}
