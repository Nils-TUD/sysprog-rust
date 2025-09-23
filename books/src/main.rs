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
    Quit,
    Help,
}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let words: Vec<&str> = command.split_whitespace().collect();
    match words.as_slice() {
        ["add", title] => Command::Add(title.to_string()),
        ["year", title, year] => Command::Year(title.to_string(), year.parse().unwrap()),
        ["rem", title] => Command::Rem(title.to_string()),
        ["show"] => Command::Show,
        ["help"] => Command::Help,
        ["quit"] => Command::Quit,
        _ => panic!("Unknown command"),
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

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        match command {
            Command::Add(title) => books.push(Book { title, year: 0 }),
            Command::Year(title, year) => {
                let book = books.iter_mut().find(|b| b.title == title).unwrap();
                book.year = year;
            },
            Command::Rem(title) => books.retain(|b| b.title != title),
            Command::Show => {
                for b in &books {
                    println!("{} ({})", b.title, b.year);
                }
            }
            Command::Help => cmd_help(),
            Command::Quit => break,
        }
    }
}
