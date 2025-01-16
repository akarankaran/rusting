use std::vec::Vec;

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 4];

    let result: Vec<String> = vec1.iter().zip(vec2.iter())
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => format!("{} is less than {}", a, b),
            std::cmp::Ordering::Greater => format!("{} is greater than {}", a, b),
            std::cmp::Ordering::Equal => format!("{} is equal to {}", a, b),
        })
        .collect();

    for res in result {
        println!("{}", res);
    }

    let vec3 = vec![3, 3, 3];
    let vec4 = vec![3, 3, 3];

    let result2: Vec<String> = vec3.iter().zip(vec4.iter())
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => format!("{} is less than {}", a, b),
            std::cmp::Ordering::Greater => format!("{} is greater than {}", a, b),
            std::cmp::Ordering::Equal => format!("{} is equal to {}", a, b),
        })
        .collect();

    for res in result2 {
        println!("{}", res);
    }

    let vec5 = vec!['a', 'b', 'c'];
    let vec6 = vec!['a', 'b', 'd'];

    let result3: Vec<String> = vec5.iter().zip(vec6.iter())
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => format!("{} is less than {}", a, b),
            std::cmp::Ordering::Greater => format!("{} is greater than {}", a, b),
            std::cmp::Ordering::Equal => format!("{} is equal to {}", a, b),
        })
        .collect();

    for res in result3 {
        println!("{}", res);
    }

    let vec7 = vec![true, false, true];
    let vec8 = vec![false, false, true];

    let result4: Vec<String> = vec7.iter().zip(vec8.iter())
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => format!("{:?} is less than {:?}", a, b),
            std::cmp::Ordering::Greater => format!("{:?} is greater than {:?}", a, b),
            std::cmp::Ordering::Equal => format!("{:?} is equal to {:?}", a, b),
        })
        .collect();

    for res in result4 {
        println!("{}", res);
    }
}