fn main() {
    let message = String::from("Greetings from earth!");
    println!("message is {message}");

    let last_word = &message[15..15+5];
    println!("last_word is {last_word}");
}
