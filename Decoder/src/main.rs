use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://docs.google.com/document/d/2PACX-1vRMx5YQlZNa3ra8dYYxmv-QIQ3YJe8tbI3kqcuC7lQiZm-CSEznKfN_HYNSpoXcZIV3Y_O3YoUB1ecq/export?format=txt";
    let body = reqwest::blocking::get(url)?.text()?;
    let doc = Html::parse_document(&body);

    let rselect = Selector::parse("tr").unwrap();
    let cselect = Selector::parse("td").unwrap();

    let mut points = Vec::<(i32, i32, char)> = Vec::new();

    for row in document.select(&rselect) {
        let cells: Vec<_> = row.select(&cselect)
            .collect();
        if cells.len() == 3 {
            let x = cells[0].text.collect::<String>.trim().parse::<i32>();
            let character = cells[1].text.collect::<String>.trim().chars().next();
            let y = cells[2].text.collect::<String>.trim().parse::<i32>();

            if let (Ok(x), some(ch), Ok(y)) = (x, character, y) {
                points.push((x, y, ch));
            }
        }
    }

    let maxX = points.iter().map(|(x, _, _)| *x).max().unwrap();
    let maxY = points.iter().map(|(_, y, _)| *x).max().unwrap();


    println!("Document content:\n{}", body);

    Ok(())
}
