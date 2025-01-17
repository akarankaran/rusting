use std::collections::HashSet;

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();
    
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    
    println!("{:?}", union);
    
    let set3: HashSet<&str> = vec!["apple", "banana", "cherry"].into_iter().collect();
    let set4: HashSet<&str> = vec!["banana", "date", "fig"].into_iter().collect();
    
    let union_str: HashSet<_> = set3.union(&set4).cloned().collect();
    
    println!("{:?}", union_str);
    
    let set5: HashSet<f64> = vec![1.1, 2.2, 3.3].into_iter().collect();
    let set6: HashSet<f64> = vec![2.2, 3.3, 4.4].into_iter().collect();
    
    let union_float: HashSet<_> = set5.union(&set6).cloned().collect();
    
    println!("{:?}", union_float);
    
    let set7: HashSet<&str> = HashSet::new();
    let set8: HashSet<&str> = vec!["one", "two", "three"].into_iter().collect();
    
    let union_empty: HashSet<_> = set7.union(&set8).cloned().collect();
    
    println!("{:?}", union_empty);
    
    let set9: HashSet<i32> = vec![].into_iter().collect();
    let set10: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
    
    let union_empty_set: HashSet<_> = set9.union(&set10).cloned().collect();
    
    println!("{:?}", union_empty_set);
}