fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}!", first_letter);

    let number: [i32; 5];
    number = [0;5];
    let index: usize = number.len();
    println!("last number is {}.", number[index]);
}
