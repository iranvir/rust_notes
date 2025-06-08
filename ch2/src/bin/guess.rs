use std::io;
use rand::Rng;

fn main() {

    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the number!");
    let mut guess = String::new();

    println!("Please input your guess:");
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
    println!("The secret numbers is: {secret_number}")
}
