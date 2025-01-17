use std::fmt;

enum Cargo {
    Wood(u32),
    Steel(u32),
    Food(u32),
}

impl fmt::Display for Cargo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cargo::Wood(ref amount) => write!(f, "Wood: {} units", amount),
            Cargo::Steel(ref amount) => write!(f, "Steel: {} units", amount),
            Cargo::Food(ref amount) => write!(f, "Food: {} units", amount),
        }
    }
}

fn process_cargo(cargo: Cargo) {
    match cargo {
        Cargo::Wood(amount) => println!("Processing {} units of wood", amount),
        Cargo::Steel(amount) => println!("Processing {} units of steel", amount),
        Cargo::Food(amount) => println!("Processing {} units of food", amount),
    }
}

fn main() {
    let cargo1 = Cargo::Wood(100);
    let cargo2 = Cargo::Steel(50);
    let cargo3 = Cargo::Food(200);

    println!("{}", cargo1);
    println!("{}", cargo2);
    println!("{}", cargo3);

    process_cargo(cargo1);
    process_cargo(cargo2);
    process_cargo(cargo3);
}