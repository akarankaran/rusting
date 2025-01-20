use std::fmt;

struct Point(i32, i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.0, self.1)
    }
}

trait Movable {
    fn move_by(&mut self, x: i32, y: i32);
}

impl Movable for Point {
    fn move_by(&mut self, x: i32, y: i32) {
        self.0 += x;
        self.1 += y;
    }
}

fn main() {
    let mut p = Point(1, 2);
    println!("{}", p);
    p.move_by(3, 4);
    println!("{}", p);
}