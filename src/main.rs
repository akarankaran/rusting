fn sum_nested_tuple(tuple: &dyn std::any::Any) -> i32 {
    match tuple.downcast_ref::<(i32, i32)>() {
        Some(&(a, b)) => a + b,
        None => {
            match tuple.downcast_ref::<((i32, i32), i32)>() {
                Some(&((a, b), c)) => a + b + c,
                None => {
                    match tuple.downcast_ref::<(i32, (i32, i32))>() {
                        Some(&(a, (b, c))) => a + b + c,
                        None => {
                            match tuple.downcast_ref::<((i32, (i32, i32)), i32)>() {
                                Some(&((a, (b, c)), d)) => a + b + c + d,
                                None => 0,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let nested_tuple_1: &(dyn std::any::Any) = &(1, 2);
    let nested_tuple_2: &(dyn std::any::Any) = &((1, 2), 3);
    let nested_tuple_3: &(dyn std::any::Any) = &(1, (2, 3));
    let nested_tuple_4: &(dyn std::any::Any) = &((1, (2, 3)), 4);
    
    println!("{}", sum_nested_tuple(nested_tuple_1));
    println!("{}", sum_nested_tuple(nested_tuple_2));
    println!("{}", sum_nested_tuple(nested_tuple_3));
    println!("{}", sum_nested_tuple(nested_tuple_4));
}