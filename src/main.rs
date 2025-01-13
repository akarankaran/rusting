use std::cmp::max;

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn largest_rectangle(rectangles: &[Rectangle]) -> Option<&Rectangle> {
    rectangles.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
}

fn main() {
    let rectangles = vec![
        Rectangle { width: 2.0, height: 3.0 },
        Rectangle { width: 5.0, height: 4.0 },
        Rectangle { width: 1.0, height: 6.0 },
        Rectangle { width: 3.0, height: 3.0 },
    ];

    if let Some(largest) = largest_rectangle(&rectangles) {
        println!("Largest area: {}", largest.area());
    } else {
        println!("No rectangles provided");
    }
}