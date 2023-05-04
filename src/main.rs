
use library_system::{ BookStatus, Library };
use library_system::Book;

fn main() {
    let library = Library {
        name: String::from("Malcom X")
    };

    let book_status = Library {
        name: String::from("Malcom X")
    };

    let book_status_clone = &book_status.clone();

    let book = Book {
        name: String::from("The Edge of AI"),
        author: String::from("Clement Adams"),
        released_date: String::from("06/01/2021"),
        status: BookStatus::Free,
        library,
    };

    println!("book_status_clone created: {:?}", book_status_clone);
    println!("booked created {:?}", &book);
}
