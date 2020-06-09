use crate::utils::choice;
use colored::*;
use std::io;

fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    5.0 * (temperature - 32.0) / 9.0
}

fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    9.0 * temperature / 5.0 + 32.0
}

pub fn start_interactive_converter() {
    println!("{}", "Fahrenheit - Celsius converter".underline());
    println!("Select the conversion:");
    println!("1) Fahrenheit -> Celsius");
    println!("2) Celsius -> Fahrenheit");
    println!("Choice:");

    let choice = choice::ask_small_integer_choice();
    println!("---");

    println!("Value to convert:");
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read the value.");
    let value: f32 = value
        .trim()
        .parse()
        .expect("There was a problem with the value.");
    println!("---\n");

    match choice {
        1 => println!(
            "{}°F equals {}°C",
            value.to_string().cyan(),
            fahrenheit_to_celsius(value).to_string().red()
        ),
        2 => println!(
            "{}°C equals {}°F",
            value.to_string().cyan(),
            celsius_to_fahrenheit(value).to_string().red()
        ),
        _ => println!("The conversion selected is incorrect."),
    }
}
