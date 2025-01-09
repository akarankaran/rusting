fn main() {
    let a = true;
    let b = false;

    let and_result = a && b;
    let or_result = a || b;
    let not_a = !a;
    let not_b = !b;

    let complex_expression_1 = (a && !b) || (!a && b);
    let complex_expression_2 = !(a || b) && (a && b);

    let values = vec![true, false];

    for &x in &values {
        for &y in &values {
            println!("x: {}, y: {}, x && y: {}, x || y: {}, !x: {}, !y: {}", x, y, x && y, x || y, !x, !y);
        }
    }

    let a_value = 5;
    let b_value = 10;
    let comparison_result = a_value < b_value;

    let is_positive = |num: i32| num > 0;
    let is_negative = |num: i32| num < 0;

    let check_positive = is_positive(3) && !is_negative(3);
    let check_negative = is_negative(-1) && !is_positive(-1);

    println!("and_result: {}", and_result);
    println!("or_result: {}", or_result);
    println!("not_a: {}", not_a);
    println!("not_b: {}", not_b);
    println!("complex_expression_1: {}", complex_expression_1);
    println!("complex_expression_2: {}", complex_expression_2);
    println!("comparison_result: {}", comparison_result);
    println!("check_positive: {}", check_positive);
    println!("check_negative: {}", check_negative);
}