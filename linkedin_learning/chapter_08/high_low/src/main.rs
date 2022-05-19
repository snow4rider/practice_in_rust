use rand::prelude::*;
use std::io;

fn main() {
    let mut guess_number = String::new();
    let number: i32 = thread_rng().gen_range(1..=10);
    println!("{}",number);

    println!("Please guess the number: ");
    io::stdin().read_line(&mut guess_number);

    let guess_number: i32 = guess_number.trim().parse().unwrap();
    if guess_number == number {
        println!("you guessed right!");
    } else if guess_number > number {
        println!("You guessed too high.");
    } else {
        println!("You guessed too low.");
    }

}
