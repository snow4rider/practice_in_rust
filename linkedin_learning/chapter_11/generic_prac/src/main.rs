struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };

    println!("rect width is {}", rect.get_width());
    
}
