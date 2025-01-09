fn main() {
    let day = "Saturday";

    match day {
        "Saturday" | "Sunday" => println!("It's the weekend!"),
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => println!("It's a weekday."),
        _ => println!("Invalid day."),
    }
}