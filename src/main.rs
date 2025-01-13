use std::io;

struct Subject {
    name: String,
    marks: u32,
}

struct Student {
    name: String,
    subjects: Vec<Subject>,
}

impl Student {
    fn total_marks(&self) -> u32 {
        self.subjects.iter().map(|subject| subject.marks).sum()
    }
}

fn main() {
    let mut student = Student {
        name: String::from("John Doe"),
        subjects: Vec::new(),
    };

    let subjects = ["Math", "Science", "English", "History", "Art"];
    
    for &subject_name in &subjects {
        println!("Enter marks for {}: ", subject_name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let marks: u32 = input.trim().parse().expect("Please enter a valid number");
        student.subjects.push(Subject {
            name: subject_name.to_string(),
            marks,
        });
    }

    let total = student.total_marks();
    println!("Total marks for {}: {}", student.name, total);
}