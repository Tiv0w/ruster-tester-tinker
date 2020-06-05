use colored::*;

fn fibonacci(number: u8) -> u32 {
    let mut previous = 0;
    let mut current = 1;
    for _ in 1..number {
        let tmp_current = current;
        current += previous;
        previous = tmp_current;
    }
    return current;
}

pub fn pretty_fibonacci(number: u8) -> String {
    format!(
        "Fibonacci sequence number at index {} is: {}.",
        number.to_string().cyan(),
        fibonacci(number).to_string().bright_purple().underline()
    )
}
