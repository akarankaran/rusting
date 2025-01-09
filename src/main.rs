fn main() {
    let a = 10;
    let b = 20;
    let c = 15;

    let largest;

    if a > b {
        if a > c {
            largest = a;
        } else {
            largest = c;
        }
    } else {
        if b > c {
            largest = b;
        } else {
            largest = c;
        }
    }

    println!("The largest number is: {}", largest);
}