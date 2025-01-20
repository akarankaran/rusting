use std::fmt;

trait Speak {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

struct Cat {
    name: String,
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

struct Cow {
    name: String,
}

impl Speak for Cow {
    fn speak(&self) -> String {
        format!("{} says: Moo!", self.name)
    }
}

struct Parrot {
    name: String,
}

impl Speak for Parrot {
    fn speak(&self) -> String {
        format!("{} says: Squawk!", self.name)
    }
}

fn main() {
    let dog = Dog { name: String::from("Rex") };
    let cat = Cat { name: String::from("Whiskers") };
    let cow = Cow { name: String::from("Bessie") };
    let parrot = Parrot { name: String::from("Polly") };

    let animals: Vec<&dyn Speak> = vec![&dog, &cat, &cow, &parrot];

    for animal in animals {
        println!("{}", animal.speak());
    }
}