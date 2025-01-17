use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle(Point, i32),
    Rectangle(Point, Point),
}

fn display_shape(shape: &Shape) {
    match shape {
        Shape::Circle(center, radius) => {
            println!("Circle at ({}, {}) with radius {}", center.x, center.y, radius);
        }
        Shape::Rectangle(top_left, bottom_right) => {
            println!("Rectangle from ({}, {}) to ({}, {})", top_left.x, top_left.y, bottom_right.x, bottom_right.y);
        }
    }
}

fn main() {
    let circle = Shape::Circle(Point { x: 1, y: 2 }, 5);
    let rectangle = Shape::Rectangle(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });

    display_shape(&circle);
    display_shape(&rectangle);

    let numbers = vec![1, 2, 3, 4, 5];
    for &num in &numbers {
        match num {
            n if n % 2 == 0 => println!("{} is even", n),
            n => println!("{} is odd", n),
        }
    }

    let tuples = vec![(1, 2), (3, 4), (5, 6)];
    for &(x, y) in &tuples {
        match (x, y) {
            (a, b) if a == b => println!("Tuple ({}, {}) has equal values", a, b),
            (a, b) => println!("Tuple ({}, {}) has different values", a, b),
        }
    }

    let coordinates: Vec<Point> = vec![Point { x: 10, y: 20 }, Point { x: -5, y: 30 }];
    for &Point { x, y } in &coordinates {
        match (x, y) {
            (x, y) if x > 0 && y > 0 => println!("Point ({}, {}) is in the first quadrant", x, y),
            (x, y) => println!("Point ({}, {}) is not in the first quadrant", x, y),
        }
    }
}