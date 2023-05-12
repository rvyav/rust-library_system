#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookStatus {
    Free,
    Booked,
}

#[derive(Debug, Clone)]
pub enum StudentBooks<T> {
    Some(T),
    None,
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
    pub books: Vec<String>
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
            println!("book {:?} has been lent to a user and removed to the library ", name);
            println!("books left and available in the library to borrow are: {:?}", current_books);
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
            books
        }
    }

    // borrowed books
    pub fn view_borrowed(&self) {
        let borrowed_books = &self.books;

        if !borrowed_books.is_empty() {
            for (idx, value) in self.books.iter().enumerate() {
                println!("book {} at index: {}", value, idx);
            }
        } else {
            println!("You have no borrowed books at the moment!.");
        }
    }

    pub fn request_book(&mut self, name: &String, library: &Library, book: &mut Book) {
        if library.books.contains(name) && (&book.name == name && book.status == BookStatus::Free) {
            // allow the library to lend the book
            library.lend_book(name);
            book.status = BookStatus::Booked;
            // add the book name to 'Student' books vector
            self.books.push(name.to_string());
        }
    }

    pub fn return_book(&mut self, name: &String, library: &mut Library, book: &mut Book) {
        if !library.books.contains(name) && (&book.name == name && book.status == BookStatus::Booked) {
            library.books.push(name.to_string());
            book.status = BookStatus::Booked;
            // remove book from 'Student' list of books
            self.books.retain(|value| value != &name.to_string());
        }
    }
}
