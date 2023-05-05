
use library_system::{ BookStatus, Library };
use library_system::Book;

fn main() {
    let library = Library {
        name: String::from("Fast Forward Learning Center")
    };

    let library_clone = library.clone();

    // let book = Book {
    //     name: String::from("The Edge of AI"),
    //     author: String::from("Clement Adams"),
    //     released_date: String::from("06/01/2021"),
    //     status: BookStatus::Free,
    //     library,
    // };

    let book = Book::new(library);

    println!("library created: {:?}", &library_clone);
    println!("booked created {:?}", &book);
}
