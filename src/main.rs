use std::collections::HashSet;

fn main() {
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();
    
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("Intersection: {:?}", intersection);

    let set3: HashSet<&str> = ["apple", "banana", "cherry"].iter().cloned().collect();
    let set4: HashSet<&str> = ["banana", "kiwi", "mango"].iter().cloned().collect();
    
    let intersection_str: HashSet<_> = set3.intersection(&set4).collect();
    println!("Intersection of strings: {:?}", intersection_str);

    let set5: HashSet<f64> = [1.1, 2.2, 3.3].iter().cloned().collect();
    let set6: HashSet<f64> = [2.2, 3.3, 4.4].iter().cloned().collect();
    
    let intersection_f64: HashSet<_> = set5.intersection(&set6).collect();
    println!("Intersection of floats: {:?}", intersection_f64);

    let set7: HashSet<String> = ["rust", "go", "python"].iter().map(|s| s.to_string()).collect();
    let set8: HashSet<String> = ["python", "java", "rust"].iter().map(|s| s.to_string()).collect();
    
    let intersection_string: HashSet<_> = set7.intersection(&set8).collect();
    println!("Intersection of String: {:?}", intersection_string);
}