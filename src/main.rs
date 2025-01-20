use std::fmt;

trait Description {
    fn describe(&self) -> String;
}

enum Vehicle {
    Car(String),
    Truck(String),
    Motorcycle(String),
}

impl Description for Vehicle {
    fn describe(&self) -> String {
        match self {
            Vehicle::Car(name) => format!("This is a car named: {}", name),
            Vehicle::Truck(name) => format!("This is a truck named: {}", name),
            Vehicle::Motorcycle(name) => format!("This is a motorcycle named: {}", name),
        }
    }
}

fn main() {
    let my_vehicle = Vehicle::Car(String::from("Toyota"));
    println!("{}", my_vehicle.describe());

    let my_truck = Vehicle::Truck(String::from("Ford"));
    println!("{}", my_truck.describe());

    let my_motorcycle = Vehicle::Motorcycle(String::from("Harley"));
    println!("{}", my_motorcycle.describe());
}