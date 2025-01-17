fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    if let (Some(first), Some(last)) = (vec1.first(), vec1.last()) {
        println!("First: {}, Last: {}", first, last);
    }

    let vec2: Vec<&str> = vec!["a", "b", "c"];
    if let (Some(first), Some(last)) = (vec2.first(), vec2.last()) {
        println!("First: {}, Last: {}", first, last);
    }

    let vec3: Vec<f64> = vec![3.14, 1.59, 2.65];
    if let (Some(first), Some(last)) = (vec3.first(), vec3.last()) {
        println!("First: {}, Last: {}", first, last);
    }

    let vec4: Vec<char> = vec!['x', 'y', 'z'];
    if let (Some(first), Some(last)) = (vec4.first(), vec4.last()) {
        println!("First: {}, Last: {}", first, last);
    }

    let vec5: Vec<i32> = Vec::new();
    if let (Some(first), Some(last)) = (vec5.first(), vec5.last()) {
        println!("First: {}, Last: {}", first, last);
    } else {
        println!("Vector is empty");
    }

    let vec6 = (1..=10).collect::<Vec<_>>();
    if let (Some(first), Some(last)) = (vec6.first(), vec6.last()) {
        println!("First: {}, Last: {}", first, last);
    }
}