use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);
    scores.insert("Charlie", 40);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let additional_scores = vec![
        ("David", 20),
        ("Eve", 60),
    ];

    for (key, value) in additional_scores {
        scores.insert(key, value);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let filtered_scores: Vec<_> = scores.iter()
        .filter(|&(_, &v)| v > 40)
        .collect();

    for (key, value) in filtered_scores {
        println!("{}: {}", key, value);
    }

    let total_score: i32 = scores.values().sum();
    println!("Total Score: {}", total_score);
}