use std::cmp;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_fit_inside(&self, other: &Rectangle) -> bool {
        (self.width <= other.width && self.height <= other.height) ||
        (self.height <= other.width && self.width <= other.height)
    }
}

fn main() {
    let rect1 = Rectangle { width: 3, height: 4 };
    let rect2 = Rectangle { width: 5, height: 6 };
    let rect3 = Rectangle { width: 6, height: 5 };
    
    assert_eq!(rect1.can_fit_inside(&rect2), true);
    assert_eq!(rect1.can_fit_inside(&rect3), true);
    assert_eq!(rect2.can_fit_inside(&rect1), false);
    assert_eq!(rect2.can_fit_inside(&rect3), true);
    assert_eq!(rect3.can_fit_inside(&rect2), false);
    assert_eq!(rect3.can_fit_inside(&rect1), false);
}