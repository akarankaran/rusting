fn main() {
    let tuple1 = (1, 2, 3);
    let tuple2 = (4, 5, 6);
    let tuple3 = (7, 8, 9);
    
    let tuples = vec![tuple1, tuple2, tuple3];
    
    for item in tuples {
        match item {
            (x, y, z) => println!("Values are: x = {}, y = {}, z = {}", x, y, z),
        }
    }
    
    let mixed_tuples: Vec<(i32, i32, i32)> = vec![(10, 20, 30), (40, 50, 60), (70, 80, 90)];
    
    for item in mixed_tuples {
        match item {
            (a, b, c) if a < 50 => println!("Small tuple: a = {}, b = {}, c = {}", a, b, c),
            (a, b, c) => println!("Large tuple: a = {}, b = {}, c = {}", a, b, c),
        }
    }
    
    let option_tuple: Option<(i32, i32, i32)> = Some((100, 200, 300));
    
    match option_tuple {
        Some((first, second, third)) => println!("Unwrapped tuple: first = {}, second = {}, third = {}", first, second, third),
        None => println!("No tuple available"),
    }
    
    let another_tuple = (1, "text", true);
    
    match another_tuple {
        (number, text, flag) => println!("Tuple contains: number = {}, text = {}, flag = {}", number, text, flag),
    }
}