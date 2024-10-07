/// For this task I have looked at multiple options and crates to parse XML in Rust.
/// This blog post was a great starting point
/// https://mainmatter.com/blog/2020/12/31/xml-and-rust/
/// It seems that an automatic parsing of the XML document to extract a schmea is not possible in Rust
/// Therefore I opted for a solution explicitly defining the schema with strong-xml and struct types

use reqwest::blocking::get;
use strong_xml::{XmlRead, XmlWrite};
use std::error::Error;
use std::borrow::Cow;

// Define the XML structure for Bookstore
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bookstore")]
struct Bookstore<'a> {
    #[xml(child = "book")]
    books: Vec<Book<'a>>, // A bookstore can have 0 to N books
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "book")]
struct Book<'a> {
    #[xml(child = "title")]
    title: Title<'a>, // The title contains an attribute, hence we must define a separate struct
    #[xml(child = "author")]
    authors: Vec<Author<'a>>,  // A book can have 0 to N books
    #[xml(child = "year")]
    year: Cow<'a, str>,
    #[xml(child = "price")]
    price: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "title")]
struct Title {
    #[xml(attr = "lang")]  
    lang: Option<Cow<str>>, // XML standard is to make attributes optional, hence Option<>
    #[xml(text)] // This is the content of the title element, its text
    name: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "author")]
struct Author {
    #[xml(text)]
    name: Cow<'a, str>, // Each author element has a simple text content
}

// The return signature Result<(), Box<dyn Error>> is, according to ChatGPT and sources, the best practice to return any kind of error, wihtout having to specify the exact type of error
fn main() -> Result<(), Box<dyn Error>> {
    // Fetch the XML data
    let url = "https://www.w3schools.com/xml/books.xml";
    let response = get(url)?.text()?;

    // Parse the XML data using strong-xml. Since all our defined structs implement XmlRead, XmlWrite trait, we can use the from_str method to parse the XML data
    let bookstore: Bookstore = Bookstore::from_str(&response)?;

    // Print the parsed data
    for book in bookstore.books {
        println!("Title: {}", book.title.name);
        if let Some(lang) = &book.title.lang {
            println!("Language: {}", lang);
        }
        for author in &book.authors {
            println!("Author: {}", author.name);
        }
        println!("Year: {}, Price: {}", book.year, book.price);
        println!();
    }

    Ok(()) // Return Ok if everything went well, this is needed due to our function signature return type
}
