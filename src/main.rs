#[allow(dead_code)]
mod cli;
mod error;

use crate::cli::Cli;
use crate::error::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    println!("Args: {:#?}", cli);
    let pattern = &cli.filter.map(|val| val.to_lowercase());

    let client = Client::new();
    let response: Vec<Ssr> = client
        .get(cli.url)
        .send()
        .await?
        .json::<Vec<Ssr>>()
        .await?
        .into_iter()
        .filter(|record| match pattern {
            Some(value) => record.name.to_lowercase().contains(value),
            None => true,
        })
        .collect();

    dbg!(&response);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Ssr {
    name: String,
    description: String,
    key: String,
    url: String,
}
