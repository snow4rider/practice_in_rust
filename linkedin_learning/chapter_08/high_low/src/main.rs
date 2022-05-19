use rand::prelude::*;
use std::io;

fn main() {
    
    let number: i32 = thread_rng().gen_range(1..=10);
    //println!("{}",number);

    loop {
        let mut guess_number = String::new();
        println!("Please guess the number: ");
        io::stdin().read_line(&mut guess_number)
                   .expect("Failed to parse the guess.");
        
        let guess_number: i32 = guess_number.trim().parse().unwrap();

        if guess_number == number {
            println!("you guessed right!");
            break;
        } else if guess_number > number {
            println!("You guessed too high.");
        } else {
            println!("You guessed too low.");
        }
    }
}
