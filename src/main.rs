use std::io;

struct Student {
    name: String,
    marks: f32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let mut input = String::new();

    loop {
        println!("Enter student name (or type 'exit' to finish):");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();
        if name.to_lowercase() == "exit" {
            break;
        }

        input.clear();
        println!("Enter marks for {}:", name);
        io::stdin().read_line(&mut input).unwrap();
        let marks: f32 = input.trim().parse().unwrap();

        students.push(Student { name, marks });
        input.clear();
    }

    println!("Student Data:");
    for student in &students {
        println!("Name: {}, Marks: {}", student.name, student.marks);
    }

    let total_marks: f32 = students.iter().map(|s| s.marks).sum();
    let average_marks = total_marks / students.len() as f32;
    println!("Average Marks: {}", average_marks);
}