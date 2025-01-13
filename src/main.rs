use std::fmt;

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}, Author: {}, Pages: {}", self.title, self.author, self.pages)
    }
}

fn print_book_details(book: &Book) {
    println!("{}", book);
}

fn main() {
    let book1 = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        pages: 328,
    };

    let book2 = Book {
        title: String::from("The Catcher in the Rye"),
        author: String::from("J.D. Salinger"),
        pages: 277,
    };

    print_book_details(&book1);
    print_book_details(&book2);
}