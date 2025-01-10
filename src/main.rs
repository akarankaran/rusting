fn main() {
    let mut vec = vec![1, 2, 3];
    add_elements(&mut vec);
    println!("{:?}", vec);
}

fn add_elements(vec: &mut Vec<i32>) {
    vec.push(4);
    vec.push(5);
    vec.push(6);
}