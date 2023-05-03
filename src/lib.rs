#[derive(Debug, Clone)]
pub enum Status {
    Free,
    Booked,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub name: String,
}

// impl Copy for Library {}

// impl Clone for Library {
//     fn clone(&self) -> Library {
//         *self
//     }
// }

#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub released_date: String,
    pub library: Library,
}

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
