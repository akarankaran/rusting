fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };

    match point {
        Point { x, y } if x > 0 && y > 0 => println!("Point is in the first quadrant: ({}, {})", x, y),
        Point { x, y } if x < 0 && y > 0 => println!("Point is in the second quadrant: ({}, {})", x, y),
        Point { x, y } if x < 0 && y < 0 => println!("Point is in the third quadrant: ({}, {})", x, y),
        Point { x, y } if x > 0 && y < 0 => println!("Point is in the fourth quadrant: ({}, {})", x, y),
        Point { x, y } => println!("Point is on an axis or origin: ({}, {})", x, y),
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Green;

    match color {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle { width: 30, height: 50 };

    match rect {
        Rectangle { width, height } if width == height => println!("Square of size: {}", width),
        Rectangle { width, height } => println!("Rectangle of size: {} x {}", width, height),
    }

    enum Shape {
        Circle { radius: f64 },
        Square { side: f64 },
    }

    let shape = Shape::Circle { radius: 2.5 };

    match shape {
        Shape::Circle { radius } => println!("Circle with radius: {}", radius),
        Shape::Square { side } => println!("Square with side: {}", side),
    }

    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    match person {
        Person { name, age } if age >= 18 => println!("{} is an adult.", name),
        Person { name, age } => println!("{} is not an adult.", name),
    }
}