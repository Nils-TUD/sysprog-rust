use std::fmt;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

struct Book {
    title: String,
    year: Option<i32>,
}

impl Book {
    fn new(title: String) -> Book {
        Book { title, year: None }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book({}", self.title)?;
        if let Some(y) = self.year {
            write!(f, ", published {}", y)?;
        }
        write!(f, ")")
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

#[derive(Debug)]
enum Error {
    MissingArg,
    UnknownCmd,
    ReadFailed,
    ParseFailed,
    NotFound,
    Exists,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ParseFailed
    }
}

fn parse(cmd: String) -> Result<Command, Error> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() == 0 {
        return Err(Error::MissingArg);
    }

    match parts[0] {
        "add" => {
            if parts.len() != 2 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Add(parts[1].to_string()))
        }

        "year" => {
            if parts.len() != 3 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Year(parts[1].to_string(), parts[2].to_string()))
        }

        "rem" => {
            if parts.len() != 2 {
                return Err(Error::MissingArg);
            }
            Ok(Command::Remove(parts[1].to_string()))
        }

        "show" => Ok(Command::Show),
        "help" => Ok(Command::Help),
        "quit" => Ok(Command::Quit),

        _ => Err(Error::UnknownCmd),
    }
}

fn get_cmd() -> Result<Command, Error> {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
        Ok(_) => parse(command.to_lowercase()),
        Err(_) => Err(Error::ReadFailed),
    }
}

fn cmd_add(books: &mut Vec<Book>, title: String) -> Result<(), Error> {
    if books.iter().find(|p| p.title == title).is_some() {
        return Err(Error::Exists);
    }

    books.push(Book::new(title));
    Ok(())
}

fn cmd_year(books: &mut Vec<Book>, title: String, year: String) -> Result<(), Error> {
    let p = books
        .iter_mut()
        .find(|p| p.title == title)
        .ok_or(Error::NotFound)?;
    p.year = Some(year.parse::<i32>()?);
    Ok(())
}

fn cmd_remove(books: &mut Vec<Book>, title: String) -> Result<(), Error> {
    books
        .iter()
        .find(|p| p.title == title)
        .ok_or(Error::NotFound)?;
    books.retain(|p| p.title != title);
    Ok(())
}

fn cmd_show(books: &[Book]) -> Result<(), Error> {
    println!("[");
    for p in books {
        println!("  {}", p);
    }
    println!("]");
    Ok(())
}

fn cmd_help() -> Result<(), Error> {
    println!("The following commands are available:");
    println!("  add <title>");
    println!("  year <title> <year>");
    println!("  rem <title>");
    println!("  show");
    println!("  help");
    println!("  quit");
    Ok(())
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        let res = match command {
            Ok(Command::Add(n)) => cmd_add(&mut books, n),
            Ok(Command::Year(n, y)) => cmd_year(&mut books, n, y),
            Ok(Command::Remove(n)) => cmd_remove(&mut books, n),
            Ok(Command::Show) => cmd_show(&books),
            Ok(Command::Help) => cmd_help(),
            Ok(Command::Quit) => break,
            Err(e) => Err(e),
        };

        if let Err(e) = res {
            println!("An error occurred: {:?}", e)
        }
    }
}
