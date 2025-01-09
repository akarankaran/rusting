fn main() {
    let grade = 85;

    match grade {
        90..=100 => println!("Excellent"),
        75..=89 => println!("Good"),
        0..=74 => println!("Poor"),
        _ => println!("Invalid grade"),
    }
}