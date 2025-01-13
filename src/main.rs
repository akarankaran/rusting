use std::io;

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter width of the rectangle: ");
    io::stdin().read_line(&mut input).unwrap();
    let width: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height of the rectangle: ");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    let rectangle = Rectangle { width, height };
    println!("The perimeter of the rectangle is: {}", rectangle.perimeter());
}