use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 75);
    scores.insert("Charlie", 85);
    
    let max_key = scores.iter().max_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap();
    println!("Key with maximum value: {}", max_key);

    let mut ages = HashMap::new();
    ages.insert("Tom", 30);
    ages.insert("Jerry", 25);
    ages.insert("Spike", 35);
    
    let max_age_key = ages.iter().max_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap();
    println!("Key with maximum age: {}", max_age_key);

    let mut temperatures = HashMap::new();
    temperatures.insert("Monday", 20);
    temperatures.insert("Tuesday", 25);
    temperatures.insert("Wednesday", 18);
    
    let max_temp_day = temperatures.iter().max_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap();
    println!("Day with maximum temperature: {}", max_temp_day);

    let mut scores2 = HashMap::new();
    scores2.insert("Player1", 500);
    scores2.insert("Player2", 700);
    scores2.insert("Player3", 600);
    
    let max_score_player = scores2.iter().max_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap();
    println!("Player with maximum score: {}", max_score_player);
}