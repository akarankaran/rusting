use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

impl Circle {
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    println!("Circumference of the circle: {}", circle.circumference());
}