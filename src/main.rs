fn main() {
    let a = true;
    let b = false;
    let c = true;

    let and_result = a && b;
    let or_result = a || b;
    let not_result = !a;

    let combined_result = a && b || c;
    let nested_result = (a && b) || (c && !b);

    let values = vec![a, b, c];
    let all_true = values.iter().all(|&x| x);
    let any_true = values.iter().any(|&x| x);

    let xor_result = a ^ b;
    let double_negation = !!b;

    let complex_expression = (a || !b) && (c && !a);
    let reachability = (a && !c) || (b || c);

    println!("AND Result: {}", and_result);
    println!("OR Result: {}", or_result);
    println!("NOT Result: {}", not_result);
    println!("Combined Result: {}", combined_result);
    println!("Nested Result: {}", nested_result);
    println!("All True: {}", all_true);
    println!("Any True: {}", any_true);
    println!("XOR Result: {}", xor_result);
    println!("Double Negation: {}", double_negation);
    println!("Complex Expression: {}", complex_expression);
    println!("Reachability: {}", reachability);
}