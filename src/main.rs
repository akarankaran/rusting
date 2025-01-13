use std::ops::Add;

struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let vec1 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let vec2 = Vector3D { x: 4.0, y: 5.0, z: 6.0 };
    let result = vec1 + vec2;

    println!("Resulting Vector: ({}, {}, {})", result.x, result.y, result.z);
}