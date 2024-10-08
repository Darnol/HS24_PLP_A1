/// For this task I have looked at multiple options and crates to parse XML in Rust.
/// This blog post was a great starting point
/// https://mainmatter.com/blog/2020/12/31/xml-and-rust/

/// First I tried to use minidom to automatically parse the XML document and extract a schema
/// Unfortunately that did not work, and given the poor documentation of minidom, I moved on

/// Then I considered using the strong-xml crate, but I did not manage to get an intuitive grip on understanding how to use it

/// Finally I tried yaserde, which to me offered a good documentaiton and a straightforward way to parse XML documents.
/// I define structs that represent the elements of the XML document I want to parse, and then I use yaserde to parse the XML document into these structs. NOTE on attributes: XML attributes in general can not be well represented in Rust structs, so I transform them into child elements. For example: 'category' of a book element simply becomes a field in the book struct. Same for title and 'lang'.

use reqwest::blocking::get;
use yaserde::YaDeserialize;
use yaserde_derive::YaDeserialize;

// Define the structs that make up the elements of the XML document we want to parse
#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "bookstore")]
struct Bookstore {
    #[yaserde(rename = "book")]
    books: Vec<Book>,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "book")]
struct Book {
    #[yaserde(attribute)]
    category: String,

    #[yaserde(child)]
    title: Title, // There can be exactly one title

    #[yaserde(rename = "author")]
    authors: Vec<String>, // There can be multiple authors

    #[yaserde(child)]
    year: i32,
    
    #[yaserde(child)]
    price: f32,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "title")]
struct Title {
    #[yaserde(attribute)]
    lang: String,
    #[yaserde(text)]
    title: String,
}

fn main() {
    // First, fetch the XML data
    let url = "https://www.w3schools.com/xml/books.xml";
    let response = get(url).expect("Error fetching the XML data").text().expect("Error parsing the XML data");
    // println!("{}", response);

    // Then try to parse the XML using yaserde
    let bookstore: Bookstore = yaserde::de::from_str(&response).expect("Failed to parse the donwloaded XML data");
    // println!("{:#?}", bookstore);

    // Process the parsed data structure
    let mut total_price: f32 = 0.0; // Summarise total price of all books
    let mut n_books: u32 = 0; // Count the number of books
    for book in bookstore.books {
        n_books += 1;
        total_price += book.price;
    }

    println!("Total price of all books: {}", total_price);
    println!("Number of books: {}", n_books);
}