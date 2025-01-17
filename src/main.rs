fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Debug)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
    }

    #[derive(Debug)]
    struct Order {
        product_id: u32,
        quantity: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let product = Product {
        id: 1,
        name: String::from("Widget"),
        price: 19.99,
    };

    let order = Order {
        product_id: 1,
        quantity: 3,
    };

    match person {
        Person { name, age: 30 } if name == "Alice" => println!("Matched Alice, age 30"),
        _ => println!("Person did not match"),
    }

    match product {
        Product { id, price, .. } if price > 20.0 => println!("Expensive product with ID: {}", id),
        Product { id, .. } => println!("Affordable product with ID: {}", id),
    }

    match order {
        Order { product_id, quantity } if quantity > 5 => println!("Large order of product ID: {}", product_id),
        Order { product_id, quantity } => println!("Order of {} units for product ID: {}", quantity, product_id),
    }

    let items: Vec<Person> = vec![
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 30 },
    ];

    for item in items {
        match item {
            Person { name, age } if age < 30 => println!("Young person: {}", name),
            Person { name, age } => println!("Adult person: {} (Age: {})", name, age),
        }
    }
}