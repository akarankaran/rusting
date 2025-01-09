fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = "Now I'm a string!";
    println!("The value of x is: {}", x);

    let x = 3.14;
    println!("Now x is a floating point number: {}", x);

    let x = true;
    println!("And now x is a boolean: {}", x);

    let x = vec![1, 2, 3];
    println!("Now x is a vector: {:?}", x);

    let x = (1, 'a', 3.0);
    println!("Now x is a tuple: {:?}", x);

    let x = char::from(97);
    println!("Now x is a char: {}", x);
}