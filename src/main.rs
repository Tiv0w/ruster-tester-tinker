mod fahrenheit_celsius;
mod fibonacci;
mod guess_the_number;
mod utils;
use crate::utils::choice;
use colored::*;

fn main() {
    println!("{}", "\nWelcome to my Toolbox!\n".purple().bold());
    println!("Select the tool you want:");
    println!("1) Fibonacci");
    println!("2) Guess The Number");
    println!("3) Fahrenheit - Celsius converter");
    println!("---");
    println!("Choice:");

    let choice = choice::ask_small_integer_choice();
    println!("---\n");

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
