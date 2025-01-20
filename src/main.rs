use std::fmt::Debug;

trait Printable {
    fn print(&self);
    
    fn print_debug(&self) where Self: Debug {
        println!("{:?}", self);
    }
}

struct Data<T> {
    value: T,
}

impl<T: Debug> Printable for Data<T> {
    fn print(&self) {
        println!("Value: {:?}", self.value);
        self.print_debug();
    }
}

fn main() {
    let data = Data { value: 42 };
    data.print();
    
    let text_data = Data { value: "Hello, Rust!" };
    text_data.print();
}