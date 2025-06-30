use colored::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number");
    let secret_number = rand::random_range(1..101);

    loop {
        println!("Please inter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Secret number was: {}", secret_number);
        println!("You guessed: {guess}",);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
