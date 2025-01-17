fn main() {
    let ages = vec![12, 15, 20, 25, 35, 60];

    for age in ages {
        let label = match age {
            0..=12 => "Child",
            13..=19 => "Teen",
            20..=34 => "Adult",
            35..=64 => "Middle-aged",
            _ => "Senior",
        };
        println!("Age: {}, Label: {}", age, label);
    }

    let ranges = vec![(0..=5, "Infant"), (6..=12, "Child"), (13..=17, "Teen"), (18..=64, "Adult"), (65..=100, "Senior")];

    let age_check = 8;
    for (range, label) in ranges {
        if range.contains(&age_check) {
            println!("Age: {}, Label: {}", age_check, label);
        }
    }

    let ages_with_labels = vec![(11, "Child"), (16, "Teen"), (23, "Adult"), (40, "Middle-aged"), (70, "Senior")];

    for (age, expected_label) in ages_with_labels {
        let result_label = match age {
            0..=12 => "Child",
            13..=19 => "Teen",
            20..=34 => "Adult",
            35..=64 => "Middle-aged",
            _ => "Senior",
        };
        println!("Age: {}, Expected: {}, Result: {}", age, expected_label, result_label);
    }
}