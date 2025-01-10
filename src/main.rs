fn double_value(num: &mut i32) {
    *num *= 2;
}

fn main() {
    let mut my_number = 5;
    double_value(&mut my_number);
    println!("{}", my_number);
    
    let mut another_number = 10;
    double_value(&mut another_number);
    println!("{}", another_number);
    
    let mut negative_number = -3;
    double_value(&mut negative_number);
    println!("{}", negative_number);
    
    let mut zero_number = 0;
    double_value(&mut zero_number);
    println!("{}", zero_number);
    
    let mut large_number = 1000;
    double_value(&mut large_number);
    println!("{}", large_number);
}