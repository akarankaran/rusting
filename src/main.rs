fn main() {
    let number = 29;
    let mut is_prime = true;

    if number < 2 {
        is_prime = false;
    } else {
        for i in 2..=(number as f64).sqrt() as usize {
            if number % i == 0 {
                is_prime = false;
                break;
            }
        }
    }

    if is_prime {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}