
use library_system::Library;
use library_system::Book;

fn main() {
    let library = Library {
        name: String::from("Malcom X")
    };

    let library_clone = library.clone();

    let book = Book {
        name: String::from("The Edge of AI"),
        author: String::from("Clement Adams"),
        released_date: String::from("06/01/2021"),
        library,
    };

    println!("library created: {:?}", library_clone);
    println!("booked created {:?}", book);
}
