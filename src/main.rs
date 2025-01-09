fn main() {
    let number = 5;
    let limit = 10;
    
    for i in 1..=limit {
        for j in 1..=limit {
            let product = number * i * j;
            print!("{} ", product);
        }
        println!();
    }
}