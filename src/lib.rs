#[derive(Debug, Clone)]
pub enum BookStatus {
    Free,
    Booked,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub released_date: String,
    pub status: BookStatus,
    pub library: Library,
}

impl Book {
    pub fn new(
        name: String,
        author: String,
        released_date: String,
        status: BookStatus,
        library: Library
    ) -> Self {

        Book {
            name,
            author,
            released_date,
            status,
            library,
        }
    }
}

// impl Default for Book {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Book {
//     pub fn new(book: Book) {
//         Self {

//         }
//     }
// }

// impl Clone for Book {
//     fn clone(&self) -> Book {
//         *self
//     }
// }

// impl Copy for Book {

// }

// impl Book {
//     pub fn new(name: String, author: String, released_date: String, library: Library) -> Self { Self { name, author, released_date, library } }
// }

// pub trait BookCRUD {
//     // method signatures
//     fn update_book(name: String) -> Book;
// }

// impl BookCRUD for Library {
//     fn update_book(name: String) -> Book {
//         Book { name:, author: }
//     }
// }
