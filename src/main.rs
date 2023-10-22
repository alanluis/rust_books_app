use std::io;

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

    for (index, book) in catalog.iter().enumerate() {
        println!("Book #{}:", index + 1);
        println!("Title: {}", &book.title);
        println!("Author: {}", &book.author);
        println!("Genre: {}", &book.genre);
        println!("Publication Year: {}", book.year);
    }
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
