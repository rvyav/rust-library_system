#[derive(Debug, Clone)]
pub enum BookStatus {
    Free,
    Booked,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub name: String,
    pub books: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub released_date: String,
    pub status: BookStatus,
    pub library: Library,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub books: Vec<String>,
}


impl Library {
    pub fn new(
        name: String,
        books: Vec<String>
    ) -> Self {

        Self {
            name,
            books,
        }
    }

    pub fn books_available() {
    }

    pub fn lend_book(&self, name: &String) {
        let mut current_books = self.books.clone();
        if current_books.contains(name) {
            // remove 'book name' from 
            // the list of books available
            current_books.retain(|value| value != name);
            println!("books left and available in the library to borrow: {:?}", current_books);
        } else {
            println!("book: {} not found in the library", name);
        }
    }
}


impl Book {
    pub fn new(
        name: String,
        author: String,
        released_date: String,
        status: BookStatus,
        library: Library
    ) -> Self {

        Self {
            name,
            author,
            released_date,
            status,
            library,
        }
    }

    pub fn book_details(&self) {
        println!("Book name is: {}", self.name);
        println!("Book author is: {}", self.author);
        println!("Book released_date is: {}", self.released_date);
        println!("Book status is: {:?}", self.status);
        println!("Book library is: {}", self.library.name);
    }

    pub fn check_book_status_type(&self) {
        match self.status {
            BookStatus::Booked => println!("status is BOOKED"),
            BookStatus::Free => println!("status is FREE"),
        }
    }
}


impl Student {
    pub fn new(
        name: String,
        books: Vec<String>
    ) -> Self {

        Self {
            name,
            books,
        }
    }

    // borrowed books
    pub fn view_borrowed() {
    }

    pub fn request_book() {
    }

    pub fn return_book() {
    }
}
