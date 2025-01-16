use std::collections::HashMap;

fn main() {
    let mut character_map = HashMap::new();
    character_map.insert('a', "Vowel");
    character_map.insert('e', "Vowel");
    character_map.insert('i', "Vowel");
    character_map.insert('o', "Vowel");
    character_map.insert('u', "Vowel");
    character_map.insert('b', "Consonant");
    character_map.insert('c', "Consonant");
    character_map.insert('d', "Consonant");
    character_map.insert('f', "Consonant");
    character_map.insert('g', "Consonant");
    character_map.insert('h', "Consonant");
    character_map.insert('j', "Consonant");
    character_map.insert('k', "Consonant");
    character_map.insert('l', "Consonant");
    character_map.insert('m', "Consonant");
    character_map.insert('n', "Consonant");
    character_map.insert('p', "Consonant");
    character_map.insert('q', "Consonant");
    character_map.insert('r', "Consonant");
    character_map.insert('s', "Consonant");
    character_map.insert('t', "Consonant");
    character_map.insert('v', "Consonant");
    character_map.insert('w', "Consonant");
    character_map.insert('x', "Consonant");
    character_map.insert('y', "Consonant");
    character_map.insert('z', "Consonant");

    let input_characters = vec!['a', 'b', 'e', 'z', 'x', 'r', 'o'];

    for character in input_characters {
        match character_map.get(&character) {
            Some(name) => println!("'{}' -> {}", character, name),
            None => println!("'{}' -> Unknown", character),
        }
    }
}