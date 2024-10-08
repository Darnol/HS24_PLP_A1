
use std::io::Write; // Import Write to use flush()

fn main() {

    'input_loop: loop {
        
        let mut user_input: String = String::new();
        println!("Please enter four positive integer numbers (m n x y) separated by one or more blank spaces or type 'quit'.");

        // CHATGPT recomends to flush before reading
        let _ = std::io::stdout().flush().expect("Failed to flush stdout"); // Ensures the prompt is printed before waiting for input
        let _ = std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
            
        // The trim approach is also used in the example in the book chapter 2 to trim leading and trailing whitespaces
        let trimmed_input: &str = user_input.trim();

        // Catch 'quit' early
        if trimmed_input == "quit" {
            println!("You have chosen to quit the program.");
            break 'input_loop; // terminate the loop alltogether
        }

        // Now, we want to split the trimmed_input into a vector of strings, split by whitespaces
        // Here we cant use an array, because we dont know the length of the input yet. Hence we use a Vec
        let split_input: Vec<&str> = trimmed_input.split_whitespace().collect();

        // Use match expression (most Rustacean way)
        match split_input[..] {
            // Try to parse to u32 and apply condition "First number must be smaller than second" and "Thrid and fourth must differ"
            [m,n,x,y] => {
                
                // Parse the strings to u32
                let m: u32 = match check_input_integer(m, "First number is not a valid positive integer. Try again.") {
                    Some(value) => value,
                    None => continue,
                };
                let n: u32 = match check_input_integer(n, "Second number is not a valid positive integer. Try again.") {
                    Some(value) => value,
                    None => continue,
                };
                let x: u32 = match check_input_integer(x, "Thrid number is not a valid positive integer. Try again.") {
                    Some(value) => value,
                    None => continue,
                };
                let y: u32 = match check_input_integer(y, "Fourth number is not a valid positive integer. Try again.") {
                    Some(value) => value,
                    None => continue,
                };

                // Compare first smaller than second
                if m >= n {
                    println!("First number must be smaller than the second. Try again.");
                    continue;
                }

                // Compare third and fourth differ
                if x == y {
                    println!("Third and fourth number must differ. Try again.");
                    continue;
                }

                // We use a Vec<&str> to dynamically add the result strings and finally call join on it
                // NOTE: &str is not appropriate, because we borrow the i in the loop, and &str would only borrow the reference
                let mut result_string_vector: Vec<String> = Vec::new();
                for i in m..=n {
                    
                    // Do the fizz buzz
                    // Most RUST like is to use match again
                    match (i % x, i % y) {
                        (0, 0) => result_string_vector.push("FizzBuzz".to_string()),
                        (0, _) => result_string_vector.push("Fizz".to_string()),
                        (_, 0) => result_string_vector.push("Buzz".to_string()),
                        (_, _) => result_string_vector.push(i.to_string()),
                    }
                }

                // print the final result
                println!("{}", result_string_vector.join(", "));

            },
            [] => println!("Your input is empty. Try again."),
            _ => println!("Your input is not of length four. Try again."),
        }


    }
}

fn check_input_integer(input: &str, msg: &str) -> Option<u32> {
    match input.parse() {
        Ok(num) => match num {
            // Also disallow 0
            0 => {
                println!("{}", msg);
                None // Return None for the Option return value
            },
            _ => Some(num), // Return Some(num) for the Option return value
        },
        Err(_) => {
            println!("{}", msg);
            None // Return None for the Option return value
        }
    }
}