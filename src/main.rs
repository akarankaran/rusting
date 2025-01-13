struct Polar {
    radius: f64,
    angle: f64,
}

impl Polar {
    fn to_cartesian(&self) -> (f64, f64) {
        let x = self.radius * self.angle.cos();
        let y = self.radius * self.angle.sin();
        (x, y)
    }
}

fn main() {
    let point1 = Polar { radius: 5.0, angle: 1.0 };
    let (x1, y1) = point1.to_cartesian();
    println!("Cartesian coordinates of point1: ({}, {})", x1, y1);

    let point2 = Polar { radius: 10.0, angle: 3.14 };
    let (x2, y2) = point2.to_cartesian();
    println!("Cartesian coordinates of point2: ({}, {})", x2, y2);

    let point3 = Polar { radius: 2.5, angle: 0.5236 };
    let (x3, y3) = point3.to_cartesian();
    println!("Cartesian coordinates of point3: ({}, {})", x3, y3);

    let point4 = Polar { radius: 7.0, angle: 2.356 };
    let (x4, y4) = point4.to_cartesian();
    println!("Cartesian coordinates of point4: ({}, {})", x4, y4);
}