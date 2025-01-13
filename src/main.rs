use std::f64;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let point1 = Point { x: 1.0, y: 2.0 };
    let point2 = Point { x: 4.0, y: 6.0 };
    let dist = point1.distance(&point2);
    println!("The distance between the points is: {}", dist);
}