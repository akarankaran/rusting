fn main() {
    let height = 5;
    for i in 1..=height {
        for j in 1..=i {
            print!("* ");
        }
        println!();
    }
}