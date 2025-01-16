fn main() {
    let number = 7;

    let category = match number {
        1..=10 => "Low",
        11..=20 => "Medium",
        21..=30 => "High",
        _ => "Out of range",
    };

    println!("The number {} is categorized as: {}", number, category);

    let numbers = vec![3, 15, 25, 35];

    for &num in &numbers {
        let status = match num {
            1..=10 => "Low",
            11..=20 => "Medium",
            21..=30 => "High",
            _ => "Out of range",
        };
        println!("The number {} is categorized as: {}", num, status);
    }

    let ranges = vec![(5, "Low"), (15, "Medium"), (25, "High"), (35, "Out of range")];

    for (num, expected) in ranges {
        let result = match num {
            1..=10 => "Low",
            11..=20 => "Medium",
            21..=30 => "High",
            _ => "Out of range",
        };
        assert_eq!(result, expected);
    }
}