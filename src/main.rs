use std::fmt;

trait Vehicle {
    fn drive(&self);
}

trait Cargo: Vehicle {
    fn load(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("The truck is driving.");
    }
}

impl Cargo for Truck {
    fn load(&self) {
        println!("The cargo is loaded onto the truck.");
    }
}

struct Boat;

impl Vehicle for Boat {
    fn drive(&self) {
        println!("The boat is sailing.");
    }
}

impl Cargo for Boat {
    fn load(&self) {
        println!("The cargo is loaded onto the boat.");
    }
}

fn main() {
    let my_truck = Truck;
    let my_boat = Boat;

    my_truck.drive();
    my_truck.load();

    my_boat.drive();
    my_boat.load();
}