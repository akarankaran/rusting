use std::cmp::PartialEq;

fn arrays_equal<T: PartialEq>(arr1: &[T], arr2: &[T]) -> bool {
    arr1 == arr2
}

fn main() {
    let array1 = [1, 2, 3, 4];
    let array2 = [1, 2, 3, 4];
    let array3 = [1, 2, 3];
    
    println!("Are array1 and array2 equal? {}", arrays_equal(&array1, &array2));
    println!("Are array1 and array3 equal? {}", arrays_equal(&array1, &array3));

    let array4 = ["hello", "world"];
    let array5 = ["hello", "world"];
    let array6 = ["hello", "Rust"];

    println!("Are array4 and array5 equal? {}", arrays_equal(&array4, &array5));
    println!("Are array4 and array6 equal? {}", arrays_equal(&array4, &array6));

    let array7 = [true, false, true];
    let array8 = [true, false, true];
    let array9 = [true, true, false];

    println!("Are array7 and array8 equal? {}", arrays_equal(&array7, &array8));
    println!("Are array7 and array9 equal? {}", arrays_equal(&array7, &array9));

    let array10: Vec<i32> = vec![1, 2, 3];
    let array11: Vec<i32> = vec![1, 2, 3];
    let array12: Vec<i32> = vec![1, 2];

    println!("Are array10 and array11 equal? {}", arrays_equal(&array10, &array11));
    println!("Are array10 and array12 equal? {}", arrays_equal(&array10, &array12));
}