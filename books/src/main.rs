use std::fmt::Display;
use std::io;
use std::io::Write;

struct Book {
    title: String,
    year: Option<u32>,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)?;
        if let Some(year) = self.year {
            write!(f, " ({})", year)?;
        }
        Ok(())
    }
}

enum Command {
    Add(String),
    Year(String, u32),
    Rem(String),
    Show,
    Help,
    Quit,
}

#[derive(Debug)]
enum Error {
    UnknownCommand,
    MissingArgument,
    IoError,
    ParseError,
    BookExists,
    BookNotFound,
}

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self::ParseError
    }
}

fn get_cmd() -> Result<Command, Error> {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;

    let args = command.split_whitespace().collect::<Vec<_>>();
    if args.is_empty() {
        return Err(Error::MissingArgument);
    }

    match args[0] {
        "add" => {
            if args.len() < 2 {
                return Err(Error::MissingArgument);
            }
            Ok(Command::Add(args[1].to_string()))
        },
        "year" => {
            if args.len() < 3 {
                return Err(Error::MissingArgument);
            }
            Ok(Command::Year(args[1].to_string(), args[2].parse()?))
        }
        "rem" => {
            if args.len() < 2 {
                return Err(Error::MissingArgument);
            }
            Ok(Command::Rem(args[1].to_string()))
        }
        "show" => Ok(Command::Show),
        "help" => Ok(Command::Help),
        "quit" => Ok(Command::Quit),
        _ => Err(Error::UnknownCommand),
    }
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

fn cmd_add(books: &mut Vec<Book>, title: String) -> Result<(), Error> {
    if books.iter().find(|b| b.title == title).is_some() {
        return Err(Error::BookExists);
    }

    books.push(Book {
        title,
        year: None,
    });
    Ok(())
}

fn cmd_year(books: &mut [Book], title: String, year: u32) -> Result<(), Error> {
    let Some(book) = books.iter_mut().find(|b| b.title == title) else {
        return Err(Error::BookNotFound);
    };
    book.year = Some(year);
    Ok(())
}

fn cmd_rem(books: &mut Vec<Book>, title: String) -> Result<(), Error> {
    if books.iter().find(|b| b.title == title).is_none() {
        return Err(Error::BookNotFound);
    }
    books.retain(|b| b.title != title);
    Ok(())
}

fn cmd_show(books: &Vec<Book>) -> Result<(), Error> {
    for b in books {
        println!("{}", b);
    }
    Ok(())
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();
        let res = match command {
            Ok(Command::Add(title)) => cmd_add(&mut books, title),
            Ok(Command::Year(title, year)) => cmd_year(&mut books, title, year),
            Ok(Command::Rem(title)) => cmd_rem(&mut books, title),
            Ok(Command::Show) => cmd_show(&books),
            Ok(Command::Help) => cmd_help(),
            Ok(Command::Quit) => break,
            Err(e) => Err(e),
        };
        if let Err(e) = res {
            println!("Error: {:?}", e);
        }
    }
}
