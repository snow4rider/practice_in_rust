use std::fs;

fn main() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);
}
