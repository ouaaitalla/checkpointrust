
#[derive(Debug)]
pub struct Writer {
    first_name: String,
    last_name: String,
    books: Vec<Book>,
}

#[derive(Debug)]
pub struct Book {
    title: String,
    year: u32,
}
pub fn order_books(writer: &mut Writer) {
    let mut books = writer.books.clone();
    books.sort_by(|a, b| a.year.cmp(&b.year));
    writer.books = books;
}
