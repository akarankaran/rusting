use std::io;

struct MutableStruct {
    field1: i32,
    field2: String,
}

impl MutableStruct {
    fn new(field1: i32, field2: String) -> Self {
        Self { field1, field2 }
    }

    fn modify_field1(&mut self, value: i32) {
        self.field1 = value;
    }

    fn modify_field2(&mut self, value: String) {
        self.field2 = value;
    }

    fn display(&self) {
        println!("Field1: {}, Field2: {}", self.field1, self.field2);
    }
}

fn main() {
    let mut my_struct = MutableStruct::new(10, String::from("Initial"));

    my_struct.display();

    my_struct.modify_field1(20);
    my_struct.modify_field2(String::from("Modified"));

    my_struct.display();
}