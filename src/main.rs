use std::collections::HashMap;

struct Leaderboard {
    scores: HashMap<String, i32>,
}

impl Leaderboard {
    fn new() -> Self {
        Leaderboard {
            scores: HashMap::new(),
        }
    }

    fn add_score(&mut self, player: String, score: i32) {
        let entry = self.scores.entry(player).or_insert(0);
        *entry += score;
    }

    fn remove_player(&mut self, player: &str) {
        self.scores.remove(player);
    }

    fn top(&self, k: usize) -> Vec<(String, i32)> {
        let mut sorted_scores: Vec<_> = self.scores.iter().collect();
        sorted_scores.sort_by(|a, b| b.1.cmp(a.1));
        sorted_scores.iter().take(k).map(|(k, v)| (k.clone(), *v)).collect()
    }

    fn reset(&mut self) {
        self.scores.clear();
    }
}

fn main() {
    let mut leaderboard = Leaderboard::new();
    leaderboard.add_score("Alice".to_string(), 50);
    leaderboard.add_score("Bob".to_string(), 30);
    leaderboard.add_score("Alice".to_string(), 40);
    leaderboard.add_score("Charlie".to_string(), 20);
    
    println!("{:?}", leaderboard.top(2));
    leaderboard.remove_player("Bob");
    println!("{:?}", leaderboard.top(2));
    leaderboard.reset();
    println!("{:?}", leaderboard.top(2));
}