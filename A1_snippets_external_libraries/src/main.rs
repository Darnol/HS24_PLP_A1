/// For this task I have looked at multiple options and crates to parse XML in Rust.
/// This blog post was a great starting point
/// https://mainmatter.com/blog/2020/12/31/xml-and-rust/

/// It seems that an automatic parsing of the XML document to extract a schmea is not possible in Rust
/// Therefore I opted for a solution explicitly defining the schema with strong-xml and struct types

use reqwest::blocking::get;
use minidom::Element;

fn main() {
    // Fetch the XML data
    let url = "https://www.w3schools.com/xml/books.xml";
    let response = get(url).expect("Error fetching the XML data").text().expect("Error parsing the XML data");
    println!("{}", response);

    // const BOOKSTORE_NS: &'static str = "bookstore";
    // let root: Element = response.parse().unwrap();

}