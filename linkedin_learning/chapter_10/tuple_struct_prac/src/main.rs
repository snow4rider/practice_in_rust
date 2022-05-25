struct Color(u8, u8, u8); // RGB


fn main() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);
    
}
