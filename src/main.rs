use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(0);
    
    println!("Deque after push operations: {:?}", deque);

    let front = deque.pop_front().unwrap();
    println!("Popped from front: {}", front);
    println!("Deque after popping front: {:?}", deque);

    let back = deque.pop_back().unwrap();
    println!("Popped from back: {}", back);
    println!("Deque after popping back: {:?}", deque);

    deque.push_back(4);
    deque.push_front(-1);
    println!("Deque after more push operations: {:?}", deque);

    let length = deque.len();
    println!("Current length of deque: {}", length);

    let is_empty = deque.is_empty();
    println!("Is the deque empty? {}", is_empty);

    for value in &deque {
        println!("Value in deque: {}", value);
    }

    deque.clear();
    println!("Deque after clear operation: {:?}", deque);
}