use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle: {} x {}", self.width, self.height)
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 15 };

    println!("{}", rect1);
    println!("Area of rect1: {}", rect1.area());
    
    println!("{}", rect2);
    println!("Area of rect2: {}", rect2.area());
    
    let rect3 = Rectangle { width: 12, height: 40 };
    println!("Area of rect3: {}", rect3.area());

    let rect4 = Rectangle { width: 20, height: 10 };
    println!("Area of rect4: {}", rect4.area());
}