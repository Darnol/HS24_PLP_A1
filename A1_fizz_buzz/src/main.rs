
use regex::Regex;
use std::io::{self, Write}; // Import Write to use flush()

fn main() {
    let mut user_input: String = String::new();
    
    println!("Please enter four positive integer numbers (m n x y) separated by one or more blank spaces or type quit.");

    // CHATGPT recomends to flush before reading
    let _ = std::io::stdout().flush(); // Ensures the prompt is printed before waiting for input

    let _ = std::io::stdin().read_line(&mut user_input).unwrap();
        
    // CHATGPT proposed this trim function so that match does not need to care about newline after read_line()
    let trimmed_input = user_input.trim();
    
    match trimmed_input {
        "quit" => {
            println!("You have chosen to quit the program.");
            return;
        },
        _ => println!("This is your input: {}", user_input),
    }
}
