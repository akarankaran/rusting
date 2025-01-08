fn compare_numbers(a: i32, b: i32) {
    if a > b {
        println!("{} is larger than {}", a, b);
    } else if b > a {
        println!("{} is larger than {}", b, a);
    } else {
        println!("Both numbers are equal.");
    }
}

fn main() {
    // Example 1: Positive numbers
    compare_numbers(10, 5);

    // Example 2: Negative and positive numbers
    compare_numbers(-3, 4);

    // Example 3: Both numbers are negative
    compare_numbers(-8, -2);

    // Example 4: Equal numbers
    compare_numbers(7, 7);

    // Example 5: Larger and smaller positive numbers
    compare_numbers(50, 20);

    // Example 6: Comparing zero
    compare_numbers(0, 0);
    compare_numbers(0, 5);
    compare_numbers(-5, 0);
}