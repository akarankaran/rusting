use std::vec::Vec;

struct Student {
    name: String,
    score: f32,
}

fn main() {
    let students: Vec<Student> = vec![
        Student { name: String::from("Alice"), score: 80.0 },
        Student { name: String::from("Bob"), score: 70.0 },
        Student { name: String::from("Charlie"), score: 85.0 },
        Student { name: String::from("David"), score: 60.0 },
        Student { name: String::from("Eve"), score: 90.0 },
    ];

    let count = students.iter().filter(|student| student.score > 75.0).count();
    println!("Number of students scoring above 75: {}", count);
}