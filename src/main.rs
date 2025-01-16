use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    let shapes: Vec<&dyn Shape> = vec![
        &Circle { radius: 3.0 },
        &Rectangle { width: 4.0, height: 5.0 },
        &Circle { radius: 2.5 },
    ];
    
    let area = total_area(&shapes);
    println!("Total area: {}", area);
}