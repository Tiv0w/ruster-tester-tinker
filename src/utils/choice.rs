use std::io;

pub fn ask_small_integer_choice() -> u8 {
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read your choice.");

    let choice: u8 = choice
        .trim()
        .parse()
        .expect("The choice selected is incorrect.");
    return choice;
}

pub fn ask_u32_integer_choice() -> u32 {
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read your choice.");

    let choice: u32 = choice
        .trim()
        .parse()
        .expect("The choice selected is incorrect.");
    return choice;
}
