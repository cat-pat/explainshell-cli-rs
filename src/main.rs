use prettytable::{format, Cell, Row, Table};
use scraper::{Html, Selector};
use std::env;
use std::error::Error;

fn build_description(url: &str, params_str: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let no_page_str: &str = "No man page found for";
    let body = response.text()?;

    if body.contains(no_page_str) {
        println!("{} {}", no_page_str, params_str);
    } else {
        let document = Html::parse_document(&body);
        let mut table = Table::new();

        table.add_row(Row::new(vec![Cell::new_align(
            params_str,
            format::Alignment::CENTER,
        )]));

        for i in 0.. {
            if let Some(description) = extract_description(&document, i) {
                if description.starts_with('-') {
                    table.add_row(Row::new(vec![Cell::new(&description)]));
                } else {
                    table.add_row(Row::new(vec![Cell::new_align(
                        &description,
                        format::Alignment::CENTER,
                    )]));
                }
            } else {
                break;
            }
        }
        table.printstd();
    }
    Ok(())
}

fn extract_description(document: &Html, index: usize) -> Option<String> {
    let selector = Selector::parse(&format!("#help-{}", index)).ok()?;
    document
        .select(&selector)
        .next()
        .map(|element| element.text().collect::<Vec<&str>>().join(" "))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Specify command to describe");
    } else {
        let params: Vec<String> = args[1..].to_vec();
        let params_str: String = params.join(" ");
        let url = format!("https://explainshell.com/explain?cmd={}", params.join("+"));
        build_description(&url, &params_str)?;
    }
    Ok(())
}
