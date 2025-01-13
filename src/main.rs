use std::cmp::PartialEq;

#[derive(Debug)]
struct Item {
    name: String,
    weight: u32,
}

#[derive(Debug)]
struct Box {
    items: Vec<Item>,
    capacity: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.weight == other.weight
    }
}

impl PartialEq for Box {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items && self.capacity == other.capacity
    }
}

fn main() {
    let item1 = Item { name: String::from("Item1"), weight: 10 };
    let item2 = Item { name: String::from("Item1"), weight: 10 };
    let item3 = Item { name: String::from("Item3"), weight: 20 };
    
    let box1 = Box { items: vec![item1.clone(), item3.clone()], capacity: 100 };
    let box2 = Box { items: vec![item2.clone(), item3.clone()], capacity: 100 };
    let box3 = Box { items: vec![item1.clone()], capacity: 50 };

    assert!(item1 == item2);
    assert!(item1 != item3);
    assert!(box1 == box2);
    assert!(box1 != box3);
}