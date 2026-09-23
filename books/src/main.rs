use std::io;
use std::io::Write;

struct Book {
    title: String,
    year: u32,
}

enum Command {
    Add(String),
    Year(String, u32),
    Rem(String),
    Show,
    Help,
    Quit,
}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let args = command.split_whitespace().collect::<Vec<_>>();
    match args[0] {
        "add" => Command::Add(args[1].to_string()),
        "year" => Command::Year(args[1].to_string(), args[2].parse().unwrap()),
        "rem" => Command::Rem(args[1].to_string()),
        "show" => Command::Show,
        "help" => Command::Help,
        "quit" => Command::Quit,
        _ => panic!("Unsupported command: {}", args[0]),
    }
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

fn cmd_add(books: &mut Vec<Book>, title: String) {
    books.push(Book {
        title,
        year: 0,
    });
}

fn cmd_year(books: &mut Vec<Book>, title: String, year: u32) {
    let book = books.iter_mut().find(|b| b.title == title).unwrap();
    book.year = year;
}

fn cmd_rem(books: &mut Vec<Book>, title: String) {
    books.retain(|b| b.title != title);
}

fn cmd_show(books: &Vec<Book>) {
    for b in books {
        println!("{} ({})", b.title, b.year);
    }
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();
        match command {
            Command::Add(title) => cmd_add(&mut books, title),
            Command::Year(title, year) => cmd_year(&mut books, title, year),
            Command::Rem(title) => cmd_rem(&mut books, title),
            Command::Show => cmd_show(&books),
            Command::Help => cmd_help(),
            Command::Quit => break,
        }
    }
}
