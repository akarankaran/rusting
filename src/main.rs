use std::collections::HashMap;
use std::io;

struct Item {
    name: String,
    quantity: u32,
    price: f32,
}

impl Item {
    fn new(name: &str, quantity: u32, price: f32) -> Item {
        Item {
            name: name.to_string(),
            quantity,
            price,
        }
    }

    fn total_value(&self) -> f32 {
        self.quantity as f32 * self.price
    }
}

fn main() {
    let mut inventory: HashMap<String, Item> = HashMap::new();
    loop {
        println!("Enter command (add, view, total, exit):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();

        match command {
            "add" => {
                let mut name = String::new();
                let mut quantity_str = String::new();
                let mut price_str = String::new();

                println!("Enter item name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                
                println!("Enter quantity:");
                io::stdin().read_line(&mut quantity_str).expect("Failed to read line");
                let quantity: u32 = quantity_str.trim().parse().expect("Please enter a valid number");

                println!("Enter price:");
                io::stdin().read_line(&mut price_str).expect("Failed to read line");
                let price: f32 = price_str.trim().parse().expect("Please enter a valid number");

                let item = Item::new(&name.trim(), quantity, price);
                inventory.insert(item.name.clone(), item);
            },
            "view" => {
                for item in inventory.values() {
                    println!("Item: {}, Quantity: {}, Price: ${:.2}", item.name, item.quantity, item.price);
                }
            },
            "total" => {
                let total_inventory_value: f32 = inventory.values().map(|item| item.total_value()).sum();
                println!("Total inventory value: ${:.2}", total_inventory_value);
            },
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}