use std::fmt;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Shape {
    name: String,
    color: Color,
    position: Point,
    volume: f64,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Shape: {}\nColor: ({}, {}, {})\nPosition: ({}, {}, {})\nVolume: {}",
            self.name,
            self.color.red,
            self.color.green,
            self.color.blue,
            self.position.x,
            self.position.y,
            self.position.z,
            self.volume
        )
    }
}

fn main() {
    let cube = Shape {
        name: String::from("Cube"),
        color: Color {
            red: 0,
            green: 255,
            blue: 0,
        },
        position: Point { x: 0.0, y: 0.0, z: 0.0 },
        volume: 27.0,
    };

    let sphere = Shape {
        name: String::from("Sphere"),
        color: Color {
            red: 255,
            green: 0,
            blue: 0,
        },
        position: Point { x: 1.0, y: 1.0, z: 1.0 },
        volume: 33.51,
    };

    println!("{}", cube);
    println!("{}", sphere);
}