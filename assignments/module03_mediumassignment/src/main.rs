use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).unwrap();
    for book in books {
        writeln!(file, "{}|{}|{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2].parse().unwrap();
            books.push(Book { title, author, year });
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "Ulysses".to_string(), author: "James Joyce".to_string(), year: 1922 },
        Book { title: "Finnegans Wake".to_string(), author: "James Joyce".to_string(), year: 1939 },
        Book { title: "The Man Without Qualities".to_string(), author: "Robert Musil".to_string(), year: 1930},
    ];
    save_books(&books, "books.txt");
    println!("Books saved to file.");
    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}