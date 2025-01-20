use std::fmt;

trait TraitExample {
    fn by_value(self);
    fn by_reference(&self);
    fn by_mut(self: &mut Self);
}

struct ExampleStruct {
    value: i32,
}

impl TraitExample for ExampleStruct {
    fn by_value(self) {
        println!("By value: {}", self.value);
    }

    fn by_reference(&self) {
        println!("By reference: {}", self.value);
    }

    fn by_mut(self: &mut Self) {
        self.value += 1;
        println!("By mutable reference: {}", self.value);
    }
}

fn main() {
    let example = ExampleStruct { value: 10 };

    example.by_reference();

    let mut mutable_example = ExampleStruct { value: 20 };
    mutable_example.by_mut();

    mutable_example.by_value();
}