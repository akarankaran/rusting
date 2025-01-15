use std::collections::HashSet;

fn main() {
    let array1 = vec![1, 2, 3, 4, 5];
    let array2 = vec![4, 5, 6, 7, 8];
    let common: HashSet<_> = array1.iter().cloned().collect();
    let result: Vec<_> = array2.iter().filter(|&&x| common.contains(&x)).collect();
    println!("{:?}", result);

    let array3 = vec!["apple", "banana", "cherry"];
    let array4 = vec!["cherry", "date", "fig", "banana"];
    let common_fruits: HashSet<_> = array3.iter().cloned().collect();
    let result_fruits: Vec<_> = array4.iter().filter(|&&x| common_fruits.contains(&x)).collect();
    println!("{:?}", result_fruits);

    let array5 = vec![1.0, 2.0, 3.0];
    let array6 = vec![3.0, 4.0, 5.0, 1.0];
    let common_floats: HashSet<_> = array5.iter().cloned().collect();
    let result_floats: Vec<_> = array6.iter().filter(|&&x| common_floats.contains(&x)).collect();
    println!("{:?}", result_floats);

    let array7 = vec![true, false, true];
    let array8 = vec![false, true];
    let common_bools: HashSet<_> = array7.iter().cloned().collect();
    let result_bools: Vec<_> = array8.iter().filter(|&&x| common_bools.contains(&x)).collect();
    println!("{:?}", result_bools);

    let array9 = vec!["red", "green", "blue"];
    let array10 = vec!["yellow", "green", "red"];
    let common_colors: HashSet<_> = array9.iter().cloned().collect();
    let result_colors: Vec<_> = array10.iter().filter(|&&x| common_colors.contains(&x)).collect();
    println!("{:?}", result_colors);
}