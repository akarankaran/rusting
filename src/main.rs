use std::ops::Mul;

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn double_dimensions(&mut self) {
        self.width *= 2.0;
        self.height *= 2.0;
    }
}

fn main() {
    let mut rect = Rectangle { width: 3.0, height: 4.0 };
    rect.double_dimensions();
    println!("New dimensions: {} x {}", rect.width, rect.height);
}