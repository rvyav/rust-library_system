
use library_system::{ BookStatus, Library };
use library_system::Book;


// static mut INSTANCE_COUNT: i32 = 0;
fn main() {

    let library = Library::new(
        String::from("Fast Forward Learning Center"),
        vec![],
    );

    let mut library_ = library.clone();

    let book = Book::new(
        String::from("The Edge of AI"),
        String::from("Clement Adams"),
        String::from("06/01/2021"),
        BookStatus::Free,
        library,
    );

    library_.books.push(String::from(&book.name));

    // unsafe {
    //     let count = books_available(INSTANCE_COUNT);
    //     println!("count: {}", count);
    // }

    // println!("library created: {:?}", &library_clone);
    // println!("booked created {:?}", &book);

    // check
    // book.book_details();

    // book.check_book_status_type();

    // println!("library clone {:?}", library_);

    // lend book
    library_.lend_book(&library_.name);

}


// fn books_available(instance_count: i32) -> i32 {
//     let instance_count = instance_count + 1;
//     instance_count
// }