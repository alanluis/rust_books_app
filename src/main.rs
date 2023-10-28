use std::io;
use prettytable::{Table, Row, Cell};
use prettytable::format;

struct Book {
    title: String,
    author: String,
    genre: String,
    year: u32,
}

fn add_book(catalog: &mut Vec<Book>) {
    println!("Add a New Book:");

    // Request book information from the user
    println!("Title: ");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");

    println!("Author: ");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read input");

    println!("Genre: ");
    let mut genre = String::new();
    io::stdin().read_line(&mut genre).expect("Failed to read input");

    println!("Publication Year: ");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("Failed to read input");
    let year: u32 = year.trim().parse().expect("Failed to parse input as an integer");

    // Create a new Book instance
    let new_book = Book {
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        genre: genre.trim().to_string(),
        year,
    };

    // Add the book to the catalog
    catalog.push(new_book);

    println!("Book added successfully!");
}

fn list_books(catalog: &Vec<Book>) {
    if catalog.is_empty() {
        println!("The catalog is empty.");
        return;
    }

    println!("List of Books:");

    // Create a new table
    let mut table = Table::new();

    // Set the table format
    table.set_format(*format::consts::FORMAT_CLEAN);

    // Add table headers
    table.add_row(Row::new(vec![
        Cell::new("Title").style_spec("bF"),
        Cell::new("Author").style_spec("bF"),
        Cell::new("Genre").style_spec("bF"),
        Cell::new("Publication Year").style_spec("bF"),
        ]));
        
        for book in catalog {
        // Add a row to the table for each book
        table.add_row(Row::new(vec![
            Cell::new(&book.title),
            Cell::new(&book.author),
            Cell::new(&book.genre),
            Cell::new(&book.year.to_string()),
        ]));
    }

    // Print the table
    table.printstd();
}

fn main() {
    let mut catalog: Vec<Book> = Vec::new();

    loop {
        println!("Choose an option:");
        println!("1. Add a book");
        println!("2. List books");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };

        match choice {
            1 => add_book(&mut catalog),
            2 => list_books(&catalog),
            3 => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}
