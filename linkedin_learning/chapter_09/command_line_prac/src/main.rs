use std::env;

fn main() {
    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }
    
}
