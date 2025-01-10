fn modify_vector(mut vec: Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        vec[i] *= 2;
    }
    vec
}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let modified_vec = modify_vector(my_vec);
    println!("{:?}", modified_vec);
}