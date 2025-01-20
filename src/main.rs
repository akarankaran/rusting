use std::f64::consts::PI;

trait Area {
    fn calculate_area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };

    println!("Circle area: {}", circle.calculate_area());
    println!("Rectangle area: {}", rectangle.calculate_area());
}