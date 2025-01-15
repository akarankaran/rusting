fn main() {
    let mut vec = Vec::new();
    
    vec.push(10);
    vec.push(20);
    vec.push(30);
    
    println!("{:?}", vec);
    
    vec.pop();
    
    println!("{:?}", vec);
    
    vec.insert(1, 15);
    
    println!("{:?}", vec);
    
    if let Some(pos) = vec.iter().position(|&x| x == 20) {
        vec.remove(pos);
    }
    
    println!("{:?}", vec);
    
    vec.extend(vec![40, 50, 60]);
    
    println!("{:?}", vec);
    
    vec.clear();
    
    println!("{:?}", vec);
    
    vec.push(100);
    vec.push(200);
    
    println!("{:?}", vec);
    
    let mut vec2 = vec.clone();
    
    vec2.sort();
    
    println!("{:?}", vec2);
    
    vec.reverse();
    
    println!("{:?}", vec);
    
    let first = vec.get(0);
    
    println!("{:?}", first);
    
    let last = vec.pop();
    
    println!("{:?}", last);
    
    vec.push(300);
    
    println!("{:?}", vec);
}