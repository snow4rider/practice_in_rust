fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14,'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    let second_item = stuff.1;

    println!("The first item is {0}\nand the second item is {1}.", first_item, second_item);

    let (a,b,c) = stuff;

    println!("a is {}\nb is {}\nc is {}", a,b,c);
}
