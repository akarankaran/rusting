fn main() {
    let mut count = 10;
    loop {
        if count == 0 {
            break;
        }
        println!("{}", count);
        count -= 1;
    }
}