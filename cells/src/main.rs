use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::rc::Rc;

struct Book {
    title: RefCell<String>,
    year: Cell<u64>,
}

impl Book {
    fn new(title: String, year: u64) -> Self {
        Self {
            title: RefCell::from(title),
            year: Cell::from(year),
        }
    }

    fn title(&self) -> Ref<'_, String> {
        self.title.borrow()
    }

    fn year(&self) -> u64 {
        self.year.get()
    }

    fn set_title(&self, title: String) {
        self.title.replace(title);
    }

    fn set_year(&self, year: u64) {
        self.year.set(year);
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
