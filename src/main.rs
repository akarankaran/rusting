fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    swap(&mut vec, 1, 3);
    println!("{:?}", vec);
}

fn swap<T>(vec: &mut Vec<T>, a: usize, b: usize) {
    let temp = std::mem::take(&mut vec[a]);
    vec[a] = std::mem::take(&mut vec[b]);
    vec[b] = temp;
}