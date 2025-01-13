use std::f64;

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Circle {
    fn fits_in_rectangle(&self, rect: &Rectangle) -> bool {
        let diameter = self.radius * 2.0;
        diameter <= rect.width && diameter <= rect.height
    }
}

fn main() {
    let circle1 = Circle { radius: 4.0 };
    let rectangle1 = Rectangle { width: 10.0, height: 10.0 };
    assert!(circle1.fits_in_rectangle(&rectangle1));

    let circle2 = Circle { radius: 6.0 };
    let rectangle2 = Rectangle { width: 5.0, height: 5.0 };
    assert!(!circle2.fits_in_rectangle(&rectangle2));
    
    let circle3 = Circle { radius: 3.0 };
    let rectangle3 = Rectangle { width: 6.0, height: 5.0 };
    assert!(circle3.fits_in_rectangle(&rectangle3));
    
    let circle4 = Circle { radius: 7.0 };
    let rectangle4 = Rectangle { width: 8.0, height: 5.0 };
    assert!(!circle4.fits_in_rectangle(&rectangle4));

    let circle5 = Circle { radius: 0.5 };
    let rectangle5 = Rectangle { width: 1.0, height: 1.0 };
    assert!(circle5.fits_in_rectangle(&rectangle5));
    
    let circle6 = Circle { radius: 10.0 };
    let rectangle6 = Rectangle { width: 20.0, height: 20.0 };
    assert!(circle6.fits_in_rectangle(&rectangle6));

    let circle7 = Circle { radius: 12.0 };
    let rectangle7 = Rectangle { width: 10.0, height: 24.0 };
    assert!(!circle7.fits_in_rectangle(&rectangle7));
}