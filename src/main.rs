mod daily_programmer;
mod small_tinkering;
mod utils;
use crate::utils::choice;
use colored::*;

fn main() {
    println!("{}", "\nWelcome to my Toolbox!\n".purple().bold());
    println!("Select the tool you want:");
    println!("1) Fibonacci");
    println!("2) Guess The Number");
    println!("3) Fahrenheit - Celsius converter");
    println!("4) String Slicer");
    println!("5) Area calculator");
    println!("6) Daily Programmer 383");
    println!("---");
    println!("Choice:");

    let choice = choice::ask_small_integer_choice();
    println!("---\n");

    match choice {
        1 => {
            for num in 1..20 {
                println!("{}", small_tinkering::fibonacci::pretty_fibonacci(num));
            }
        }
        2 => small_tinkering::guess_the_number::guess_the_number(),
        3 => small_tinkering::fahrenheit_celsius::start_interactive_converter(),
        4 => println!(
            "First and Second word: {}, {}",
            small_tinkering::slicer::first_word("yes sir"),
            small_tinkering::slicer::second_word("yes sir")
        ),
        5 => small_tinkering::rectangle::rectangle_tester(),
        6 => daily_programmer::dp_383::same_necklace(),
        _ => println!("The tool selected is incorrect."),
    }
}
