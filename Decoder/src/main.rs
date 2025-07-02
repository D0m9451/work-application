use std::collections::HashMap;
use std::error::Error;

fn render(link: &str) -> Resuly<(), Box<dyn Error>> {
    let body = reqwest::blocking::get(link)?.text()?;

    for line in body.lines() {
        
    }
}

fn main() ->  Result<(), Box<dyn Error>> {
    let link = "https://docs.google.com/document/d/e/2PACX-1vRMx5YQlZNa3ra8dYYxmv-QIQ3YJe8tbI3kqcuC7lQiZm-CSEznKfN_HYNSpoXcZIV3Y_O3YoUB1ecq/export?format=txt";
    render(link)?;
    Ok(())
}