use std::fmt;
use std::rc::Rc;

struct Book {
    title: String,
    year: u64,
}

impl Book {
    fn new(title: String, year: u64) -> Self {
        Self { title, year }
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn year(&self) -> u64 {
        self.year
    }

    fn set_title(&self, title: String) {
        // TODO
    }

    fn set_year(&self, year: u64) {
        // TODO
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}", self.title(), self.year())
    }
}

fn main() {
    let books = Rc::new(vec![
        Book::new("The Rust Programming Language".to_string(), 2022),
        Book::new("Effective Modern C++".to_string(), 2015),
        Book::new("Modern Operating System".to_string(), 2023),
    ]);

    books[1].set_year(2014);
    books[2].set_title("Modern Operating Systems".to_string());

    for b in &*books {
        println!("{}", b);
    }
}
