fn main() {
    let num1 = 10;
    let num2 = 20;
    
    if num1 > num2 {
        println!("{} is larger", num1);
    } else if num1 < num2 {
        println!("{} is larger", num2);
    } else {
        println!("Both numbers are equal");
    }

    let a = 5;
    let b = 5;
    if a > b {
        println!("{} is larger", a);
    } else if a < b {
        println!("{} is larger", b);
    } else {
        println!("Both numbers are equal");
    }

    let x = -3;
    let y = -7;
    if x > y {
        println!("{} is larger", x);
    } else if x < y {
        println!("{} is larger", y);
    } else {
        println!("Both numbers are equal");
    }

    let p = 0;
    let q = 42;
    if p > q {
        println!("{} is larger", p);
    } else if p < q {
        println!("{} is larger", q);
    } else {
        println!("Both numbers are equal");
    }

    let m = 100;
    let n = 67;
    let larger = if m > n { m } else { n };
    println!("The larger number is {}", larger);
}