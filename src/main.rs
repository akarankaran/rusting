fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    
    println!("{}", s2);
    
    let a = Box::new(5);
    let b = a;
    
    println!("{}", b);
    
    let x = String::from("Ownership");
    take_ownership(x);
    
    // Uncommenting the next line will cause an error because x has been moved
    // println!("{}", x);
    
    let y = String::from("Borrowed");
    let z = borrow_string(&y);
    
    println!("{}", y);
    println!("{}", z);
    
    let v1 = vec![1, 2, 3];
    let v2 = move_vec(v1);
    
    println!("{:?}", v2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &String) -> String {
    s.clone()
}

fn move_vec(v: Vec<i32>) -> Vec<i32> {
    v
}