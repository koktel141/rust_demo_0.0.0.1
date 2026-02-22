use std::io::{self, Write};

fn get_int(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        // Ensure the prompt is displayed before waiting for input
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Attempt to parse the trimmed string into an i32
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn main() {
    let limit = get_int("Enter the limit number: ");
    let mut sum = 0;
    println!("Counting by steps of 1.");
    for _i in 0..limit {
        let n = get_int("Enter your number:");
        sum+=n;
    }
    println!("The sum is: {}", sum);
    println!("Done!");
}