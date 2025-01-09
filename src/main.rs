fn main() {
    let mut sum = 0;
    let mut count = 0;
    let mut number = 2;

    while count < 10 {
        sum += number;
        number += 2;
        count += 1;
    }

    println!("The sum of the first 10 even numbers is: {}", sum);
}