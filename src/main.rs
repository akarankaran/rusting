use std::vec::Vec;

struct IntegerVector {
    numbers: Vec<i32>,
}

impl IntegerVector {
    fn sum(&self) -> i32 {
        self.numbers.iter().sum()
    }
}

fn main() {
    let vec = IntegerVector { numbers: vec![1, 2, 3, 4, 5] };
    let total = vec.sum();
    println!("The sum of the vector is: {}", total);
}