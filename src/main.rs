use std::fmt;

enum Vehicle {
    Car { make: String, model: String, year: u16 },
    Truck { make: String, model: String, payload_capacity: u32 },
    Motorcycle { make: String, model: String, year: u16, has_sidecar: bool },
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vehicle::Car { make, model, year } => write!(f, "Car: {} {} ({})", make, model, year),
            Vehicle::Truck { make, model, payload_capacity } => write!(f, "Truck: {} {} (Capacity: {} kg)", make, model, payload_capacity),
            Vehicle::Motorcycle { make, model, year, has_sidecar } => write!(f, "Motorcycle: {} {} ({}) - Sidecar: {}", make, model, year, has_sidecar),
        }
    }
}

fn describe_vehicle(vehicle: Vehicle) {
    match vehicle {
        Vehicle::Car { make, model, year } => println!("This is a {} {} from {}.", make, model, year),
        Vehicle::Truck { make, model, payload_capacity } => println!("This is a {} {} with a payload capacity of {} kg.", make, model, payload_capacity),
        Vehicle::Motorcycle { make, model, year, has_sidecar } => {
            let sidecar_status = if has_sidecar { "with" } else { "without" };
            println!("This is a {} {} from {} {} sidecar.", make, model, year, sidecar_status);
        },
    }
}

fn main() {
    let vehicles = vec![
        Vehicle::Car { make: String::from("Toyota"), model: String::from("Corolla"), year: 2020 },
        Vehicle::Truck { make: String::from("Ford"), model: String::from("F-150"), payload_capacity: 1000 },
        Vehicle::Motorcycle { make: String::from("Harley-Davidson"), model: String::from("Sportster"), year: 2021, has_sidecar: true },
    ];
    
    for vehicle in vehicles {
        println!("{}", vehicle);
        describe_vehicle(vehicle);
    }
}