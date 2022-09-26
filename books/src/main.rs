use std::io;
use std::io::Write;

struct Book {
    title: String,
    year: i32,
}

impl Book {
    fn new(title: String) -> Book {
        Book { title, year: 0 }
    }
}

enum Command {
    Add(String),
    Year(String, String),
    Remove(String),
    Show,
    Help,
    Quit,
}

fn parse(cmd: String) -> Command {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() == 0 {
        panic!("Unknown command");
    }

    match parts[0] {
        "add" => {
            assert!(parts.len() == 2);
            Command::Add(parts[1].to_string())
        }

        "year" => {
            assert!(parts.len() == 3);
            Command::Year(parts[1].to_string(), parts[2].to_string())
        }

        "rem" => {
            assert!(parts.len() == 2);
            Command::Remove(parts[1].to_string())
        }

        "show" => Command::Show,
        "help" => Command::Help,
        "quit" => Command::Quit,

        _ => panic!("Unknown command"),
    }
}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    parse(command)
}

fn cmd_add(books: &mut Vec<Book>, title: String) {
    books.push(Book::new(title));
}

fn cmd_year(books: &mut Vec<Book>, title: String, year: String) {
    let p = books.iter_mut().find(|p| p.title == title).unwrap();
    p.year = year.parse::<i32>().unwrap();
}

fn cmd_remove(books: &mut Vec<Book>, title: String) {
    books.retain(|p| p.title != title);
}

fn cmd_show(books: &Vec<Book>) {
    println!("[");
    for p in books {
        println!("  {} (Year {})", p.title, p.year);
    }
    println!("]");
}

fn cmd_help() {
    println!("The following commands are available:");
    println!("  add <title>");
    println!("  year <title> <year>");
    println!("  rem <title>");
    println!("  show");
    println!("  help");
    println!("  quit");
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        match command {
            Command::Add(n) => cmd_add(&mut books, n),
            Command::Year(n, y) => cmd_year(&mut books, n, y),
            Command::Remove(n) => cmd_remove(&mut books, n),
            Command::Show => cmd_show(&books),
            Command::Help => cmd_help(),
            Command::Quit => break,
        };
    }
}
