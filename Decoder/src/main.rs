use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::HashMap::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://docs.google.com/document/d/2PACX-1vRMx5YQlZNa3ra8dYYxmv-QIQ3YJe8tbI3kqcuC7lQiZm-CSEznKfN_HYNSpoXcZIV3Y_O3YoUB1ecq/export?format=txt";
    let response = get(url)?.text()?;
    println!("Document content:\n{}", response);

    Ok(())
}
