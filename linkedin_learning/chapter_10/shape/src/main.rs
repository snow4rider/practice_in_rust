struct Circle{
    radius: f64,
    x: f64,
    y: f64,
    h: f64,
    k: f64
}

impl Circle {
    fn new(radius: f64, h: f64, k: f64){


    }
}


struct Rectangle {
    length: f64,
    width: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        let area = self.length * self.width;
        return area;
    }

    fn get_per(&self) -> f64 {
        let per = 2.0 * self.length + 2.0 * self.width;
        return per;

    }

    fn scale(&mut self, scale: f64) {
        self.length *= scale;
        self.width *= scale;
    }


    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle {
            length,
            width
        }
    }
}


fn main() {
    let mut rect = Rectangle::new(1.2,3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(),1.02);
    println!("Tests Passed!");
}
