mod cli;
mod error;

use crate::cli::Cli;
use crate::error::Result;
use cli::Environment;
use reqwest::Client;
use serde::{Deserialize, Serialize};

impl crate::cli::Environment {
    pub fn parse(&self) -> (&str, &str) {
        match self {
            Environment::Dev => ("env", "dev"),
            Environment::Qa => ("env", "qa"),
            Environment::Uat => ("env", "uat"),
            Environment::Prod => ("env", "prod"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Ssr {
    name: String,
    description: String,
    key: String,
    url: String,
}

async fn get_records(client: &Client, cli: &Cli) -> Result<Vec<Ssr>> {
    let pattern = &cli.filter.clone().map(|val| val.to_lowercase());
    let target = cli.target_environment.parse();
    let result = client
        .get(&cli.url)
        .query(&[target])
        .send()
        .await?
        .json::<Vec<Ssr>>()
        .await?
        .into_iter()
        .filter(|record| match pattern {
            Some(value) => {
                record.name.to_lowercase().contains(value)
                    || record.description.to_lowercase().contains(value)
                    || record.key.to_lowercase().contains(value)
            }
            None => true,
        })
        .collect::<Vec<Ssr>>();
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let client = Client::new();
    let response2 = get_records(&client, &cli).await?;

    dbg!(&response2);

    Ok(())
}
