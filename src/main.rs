use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<_> = [4, 5, 6, 7, 8].iter().cloned().collect();
    
    let difference: HashSet<_> = set_a.difference(&set_b).collect();
    println!("Difference of set_a from set_b: {:?}", difference);

    let set_c: HashSet<_> = ["apple", "banana", "cherry"].iter().cloned().collect();
    let set_d: HashSet<_> = ["banana", "kiwi", "mango"].iter().cloned().collect();
    
    let difference_strings: HashSet<_> = set_c.difference(&set_d).collect();
    println!("Difference of set_c from set_d: {:?}", difference_strings);

    let set_e: HashSet<_> = [10, 20, 30].iter().cloned().collect();
    let set_f: HashSet<_> = [1, 10, 20].iter().cloned().collect();
    
    let difference_numeric: HashSet<_> = set_e.difference(&set_f).collect();
    println!("Difference of set_e from set_f: {:?}", difference_numeric);

    let set_g: HashSet<_> = ["red", "blue", "green"].iter().cloned().collect();
    let set_h: HashSet<_> = ["yellow", "blue", "purple"].iter().cloned().collect();
    
    let difference_colors: HashSet<_> = set_g.difference(&set_h).collect();
    println!("Difference of set_g from set_h: {:?}", difference_colors);
}