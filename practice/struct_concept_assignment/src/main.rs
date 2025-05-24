// Create a struct Book with fields: title, author, pages, available. Then:
// Create an instance of Book
// Print each field
#[derive(Debug, Default)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}
// Write a function build_book that takes title and author and returns a Book. Use field init shorthand.
fn build_book(title: String, author: String) -> Book {
    let book_store = Book {
        title,
        author,
        pages: 2,
        available: true,
    };
    book_store
}

// Add an impl block to Book with:
// Method summary(&self) that returns a String like "Title by Author"
// Method is_long(&self) that returns true if pages > 300
impl Book {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
    fn is_long(&self) -> bool {
        self.pages > 300
    }
    // Add an associated function Book::new(title, author) -> Book that sets default pages = 100 and available = true.
    fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            pages: 100,
            available: true,
        }
    }
}

// Define:
// A tuple struct RGB(u8, u8, u8)
// Create and print their instances
#[derive(Debug)]
struct RGB(u8, u8, u8);
// A unit-like struct Success
#[derive(Debug)]
struct Success;
#[derive(Debug)]
struct Rectangle {
    length: f32,
    breath: f32,
}
impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.breath
    }
    fn square(size: f32) -> Rectangle {
        Rectangle {
            length: size,
            breath: size,
        }
    }
}

fn main() {
    let book1 = Book {
        title: String::from("Rust"),
        author: String::from("James Yoda"),
        pages: 200,
        available: true,
    };
    println!("Title {}", book1.title);
    println!("author {}", book1.author);
    println!("pagese {}", book1.pages);
    println!("avaliable {}", book1.available);
    // book2 = Book::default();
    // let mut book2 = Book::default();
    // book2.title = String::from("Adavance Eng");
    // book2.author = "Basant".to_string();
    // book2.pages = 48;
    // book2.available = false;
    // println!("book 2 details is {:?}",book2);
    // println!("book 2 details is {:#?}",book2);

    // let book3 = build_book("AI", "NG");
    let book3 = build_book("AI".to_string(), "NG".to_string());
    println!("book 3 details is {:#?}", book3);

    // Given a book1, create a book4 using update syntax, but change the title.
    let book4 = Book {
        title: "Ml".to_string(),
        ..book1
    };
    println!("book 4 details is {:#?}", book4);
    // println!("book 1 details is {:#?}", book1); // ownership of book1 is transfewrd to vook 1
    println!("\n \n \n \t TUPPLE STRUCT \t ");
    let red = RGB(123, 213, 221);
    println!("red is {:?}", red);
    let status = Success;
    println!("States is {:?}", status);

    println!("BOOK with dbg");
    dbg!(&book4);
    // dbg!(book4); //take waya ownership
    println!("{}", book4.summary());
    // book4.summary(); //print nothing
    println!("{}", book4.is_long());

    let book = Book::new("Rustacean Tales".to_string(), "Ferris the Crab".to_string());

    println!("Book Summary: {}", book.summary());
    println!("Pages: {}", book.pages);
    println!("Available: {}", book.available);
    let rect1 = Rectangle {
        length: 30.0,
        breath: 50.0,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let square = Rectangle::square(20.0);
    println!("Square details: {:#?}", square);
    println!("Area of square is {}", square.area());
}
