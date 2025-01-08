fn main() {
    welcome();
}

fn welcome() {
    println!("Welcome to Rust!");
}

// Variation 1
fn main() {
    greet();
}

fn greet() {
    println!("Welcome to Rust!");
}

// Variation 2
fn main() {
    let message = create_welcome_message();
    println!("{}", message);
}

fn create_welcome_message() -> &'static str {
    "Welcome to Rust!"
}

// Variation 3
fn main() {
    print_welcome();
}

fn print_welcome() {
    println!("Welcome to Rust!");
}

// Variation 4
fn main() {
    show_message("Welcome to Rust!");
}

fn show_message(msg: &str) {
    println!("{}", msg);
}

// Variation 5
fn main() {
    display_welcome();
}

fn display_welcome() {
    let welcome_text = "Welcome to Rust!";
    println!("{}", welcome_text);
}