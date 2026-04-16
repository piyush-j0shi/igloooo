use serde::{Deserialize, Serialize};
use std::fs::{read, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
enum Availablity {
    Available,
    NotAvailable,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    availablable: Availablity,
}

impl Book {
    fn new(book_title: String, book_author: String, book_isbn: String) -> Self {
        Self {
            title: book_title,
            author: book_author,
            isbn: book_isbn,
            availablable: Availablity::Available,
        }
    }

    fn details(&self) {
        println!("book title : {:?}", self.title);
        println!("book author : {:?}", self.author);
        println!("book isbn : {:?}", self.isbn);
        println!("is available : {:?}", self.availablable);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
        save_data(self);
    }

    fn check_out(&mut self) {
        println!("enter the book you want to check out");
        let book_name = read_input();

        if let Some(book) = self
            .books
            .iter_mut()
            .find(|s| s.title.to_lowercase() == book_name.trim().to_lowercase())
        {
            book.availablable = Availablity::NotAvailable;
            save_data(&self);
            println!("{} checked out", book_name);
        } else {
            println!("book does not exists");
        }
    }

    fn return_out(&mut self) {
        println!("enter the book name you want to return");
        let book_name = read_input();

        if let Some(book) = self
            .books
            .iter_mut()
            .find(|s| s.title.to_lowercase() == book_name.trim().to_lowercase())
        {
            match book.availablable {
                Availablity::Available => println!("enter the correct book name"),
                Availablity::NotAvailable => book.availablable = Availablity::Available,
            }
            save_data(&self);
        } else {
            println!("book does not exist");
        }
    }

    fn search_by_title(&self) {
        println!("enter the book title");
        let book_name = read_input();

        if let Some(book) = self
            .books
            .iter()
            .find(|s| s.title.to_lowercase() == book_name.trim().to_lowercase())
        {
            book.details();
        } else {
            println!("book does not exist");
        }
    }

    fn search_by_author(&self) {
        println!("enter the author name");
        let author_name = read_input();

        let book: Vec<&Book> = self
            .books
            .iter()
            .filter(|s| s.author.to_lowercase() == author_name.trim().to_lowercase())
            .collect();
        if book.is_empty() {
            println!("book does not exists");
        } else {
            for single_book in book {
                single_book.details();
            }
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read lines");
    input.trim().to_string()
}

fn save_data(library: &Library) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("data1.json")
        .expect("failed to open");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, library).expect("failed to write");
}

fn load_data() -> Library {
    let file = File::open("data1.json").expect("failed to open file");
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(data) => {
            let mut library = data;
            return library;
        }

        Err(e) => {
            let mut library = Library {
                books: vec![
                    Book {
                        title: "The Rust Programming Language".to_string(),
                        author: "Steve Klabnik & Carol Nichols".to_string(),
                        isbn: "978-1-59327-828-1".to_string(),
                        availablable: Availablity::Available,
                    },
                    Book {
                        title: "Clean Code".to_string(),
                        author: "Robert C. Martin".to_string(),
                        isbn: "978-0-13-235088-4".to_string(),
                        availablable: Availablity::NotAvailable,
                    },
                    Book {
                        title: "The Pragmatic Programmer".to_string(),
                        author: "Andrew Hunt & David Thomas".to_string(),
                        isbn: "978-0-13-595705-9".to_string(),
                        availablable: Availablity::Available,
                    },
                    Book {
                        title: "Introduction to Algorithms".to_string(),
                        author: "Thomas H. Cormen".to_string(),
                        isbn: "978-0-262-03384-8".to_string(),
                        availablable: Availablity::Available,
                    },
                ],
            };
            return library;
        }
    }
}

fn execute() {
    println!("there are few options");
    println!("===========================================================================================================================");

    let mut library = load_data();
    loop {
        // let mut library = Library { books: Vec::new() };
        println!("add : add_book | check : check_out | return : return_book | auth : search_by_author | title : search_by_title | exit : exit");
        println!("===========================================================================================================================");

        let user_input = read_input().trim().to_lowercase();

        if user_input == "add" {
            println!("enter book title");
            let book_title = read_input().trim().to_lowercase();

            println!("enter book author");
            let book_author = read_input().trim().to_lowercase();

            if book_title.is_empty() {
                println!("title can not be empty");
            } else if book_author.is_empty() {
                println!("author can not be empty");
            } else {
                let book = Book::new(book_title, book_author, "1234567890123456".to_string());
                library.add_book(book);
                println!("book added");
                println!("===========================================================================================================================");
            }
        } else if user_input == "check" {
            library.check_out();
            println!("book successfully checked out");
            println!("===========================================================================================================================");
        } else if user_input == "return" {
            library.return_out();
            println!("book successfully returned out");
            println!("===========================================================================================================================");
        } else if user_input == "auth" {
            library.search_by_author();
            println!("===========================================================================================================================");
        } else if user_input == "title" {
            library.search_by_title();
            println!("===========================================================================================================================");
        } else if user_input == "exit" {
            break;
        } else {
            println!("invalid choice");
        }
    }
}

fn main() {
    execute();
}
