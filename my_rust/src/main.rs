mod calculator;

use crate::calculator::{evaluate, parse};
use std::io;

fn calculate_from_input(input: &str) -> Result<f64, String> {
    let expression = parse(input)?;
    let result = evaluate(&expression.numbers, &expression.operators)?;
    Ok(result)
}

fn main() {
    println!("Simple Calculator");
    println!("Enter an expression like '24 + 8' or type 'quit' to exit");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_lowercase().eq("quit") || input.trim().to_lowercase().eq("q") {
            println!("Exiting simple Calculator");
            break;
        }

        match calculate_from_input(&input) {
            Ok(result) => println!(" = {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}
