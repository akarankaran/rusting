use std::cmp::Ordering;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(10.0),
        Shape::Rectangle(5.0, 10.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in shapes {
        match shape {
            Shape::Circle(radius) if radius > 10.0 => println!("Large Circle with radius: {}", radius),
            Shape::Circle(radius) => println!("Circle with radius: {}", radius),
            Shape::Rectangle(width, height) if width == height => println!("Square with side: {}", width),
            Shape::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
            Shape::Triangle(a, b, c) if a + b > c && a + c > b && b + c > a => println!("Triangle with sides: {}, {}, {}", a, b, c),
            Shape::Triangle(_, _, _) => println!("Invalid Triangle"),
        }
    }

    let number = 6;
    match number {
        n if n % 2 == 0 && n < 10 => println!("Even number less than 10: {}", n),
        n if n % 2 == 0 => println!("Even number: {}", n),
        n if n < 10 => println!("Odd number less than 10: {}", n),
        n => println!("Number 10 or greater: {}", n),
    }

    let text = "hello";
    match text {
        text if text.len() < 5 => println!("Short text: {}", text),
        text if text.len() >= 5 && text.len() < 10 => println!("Medium text: {}", text),
        _ => println!("Long text: {}", text),
    }
}