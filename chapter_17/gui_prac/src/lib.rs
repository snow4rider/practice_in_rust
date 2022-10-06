pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vex<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for components in self.components.iter() {
            components.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
