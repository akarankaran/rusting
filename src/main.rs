use std::fmt;

trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    person.print();
    
    let another_person = Person {
        name: String::from("Bob"),
        age: 25,
    };

    another_person.print();    
}