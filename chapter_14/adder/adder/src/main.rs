use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world {num} plus one is {}!",
        add_one::add_one(num)
    );
    println!(
        "this is plus two {}",
        add_two::add_two(num)
    )
}