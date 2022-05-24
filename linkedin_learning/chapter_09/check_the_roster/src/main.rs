use std::env;
use std::fs;

fn main() {

    let arg01 = env::args().nth(1).unwrap();
    let arg02 = env::args().nth(2).unwrap();
    let full_name = arg01 + " " + &arg02;
    let roster = fs::read_to_string("roster.txt").unwrap();
    let mut vec = Vec::new();

    for name in roster.lines() {
        vec.push(name);
    }

    println!("{}", vec[0]);
   
    if full_name == vec[0]{
        println!("Hello there {}", vec[0]);
    }

}
