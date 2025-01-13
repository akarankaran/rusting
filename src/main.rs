fn main() {
    struct Person {
        name: String,
        age: u32,
    }

    let mut person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = Person {
        name: String::from("Bob"),
        ..person1
    };

    person1.age = 31;

    let person3 = Person {
        age: 25,
        ..person1
    };

    let person4 = Person {
        name: String::from("Charlie"),
        age: 40,
    };

    let person5 = Person {
        name: String::from("Dave"),
        ..person4
    };

    let person6 = Person {
        ..person1
    };

    let person7 = Person {
        name: String::from("Eve"),
        age: 22,
    };

    let person8 = Person {
        age: 28,
        ..person7
    };

    let person9 = Person {
        name: String::from("Frank"),
        age: 35,
    };

    let person10 = Person {
        name: String::from("Grace"),
        ..person9
    };

    println!("Person 2: {} is {} years old", person2.name, person2.age);
    println!("Person 3: {} is {} years old", person3.name, person3.age);
    println!("Person 5: {} is {} years old", person5.name, person5.age);
    println!("Person 6: {} is {} years old", person6.name, person6.age);
    println!("Person 8: {} is {} years old", person8.name, person8.age);
    println!("Person 10: {} is {} years old", person10.name, person10.age);
}