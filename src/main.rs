fn main() {
    struct Person {
        name: String,
        age: u32,
        city: String,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("New York"),
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("City: {}", person.city);
}