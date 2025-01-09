fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut count = 0;

    while count < 10 {
        println!("{}", a);
        let next = a + b;
        a = b;
        b = next;
        count += 1;
    }
}