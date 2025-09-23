use std::io;
use std::io::Write;
use std::num::ParseIntError;

struct Book {
    title: String,
    year: Option<u32>,
}

enum Command {
    Add(String),
    Year(String, u32),
    Rem(String),
    Show,
    Quit,
    Help,
}

#[derive(Debug)]
enum Error {
    UnknownCommand,
    ParseInt,
    IOError,
}

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Self::ParseInt
    }
}

fn get_cmd() -> Result<Command, Error> {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;

    let words: Vec<&str> = command.split_whitespace().collect();
    match words.as_slice() {
        ["add", title] => Ok(Command::Add(title.to_string())),
        ["year", title, year] => Ok(Command::Year(title.to_string(), year.parse()?)),
        ["rem", title] => Ok(Command::Rem(title.to_string())),
        ["show"] => Ok(Command::Show),
        ["help"] => Ok(Command::Help),
        ["quit"] => Ok(Command::Quit),
        _ => Err(Error::UnknownCommand),
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
        let command = match get_cmd() {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("An error occurred: {:?}", e);
                continue;
            }
        };

        match command {
            Command::Add(title) => books.push(Book { title, year: None }),
            Command::Year(title, year) => {
                let book = match books.iter_mut().find(|b| b.title == title) {
                    Some(book) => book,
                    None => {
                        println!("Book with title {} not found.", title);
                        continue;
                    }
                };
                book.year = Some(year);
            }
            Command::Rem(title) => books.retain(|b| b.title != title),
            Command::Show => {
                for b in &books {
                    if let Some(year) = b.year {
                        println!("{} ({})", b.title, year);
                    } else {
                        println!("{}", b.title);
                    }
                }
            }
            Command::Help => cmd_help(),
            Command::Quit => break,
        }
    }
}
