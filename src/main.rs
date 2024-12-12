use anyhow::{Ok, Result};
use select::document::Document;
use select::predicate::Name;
use std::env::{self};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Page not provided");
        return Ok(());
    }

    let page_url = &args[1];

    let res = reqwest::get(page_url).await?.text().await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{x}"));

    Ok(())
}
