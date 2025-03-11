mod cli;
mod error;

use crate::cli::Cli;
use crate::error::Result;
use cli::Environment;
use error::Error;
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

async fn get_records(
    client: &reqwest::RequestBuilder,
    target: &(&str, &str),
    pattern: &Option<String>,
) -> Result<Vec<Ssr>> {
    let result = client
        .try_clone()
        .ok_or_else(|| Error::custom("Unable to clone client"))?
        .query(&[target])
        .send()
        .await?
        .json::<Vec<Ssr>>()
        .await?
        .into_iter()
        .filter(|record| match &pattern {
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
    let client = Client::new().get(&cli.url);
    let pattern = &cli.filter.clone().map(|val| val.to_lowercase());
    let target = cli.target_environment.parse();
    let response2 = get_records(&client, &target, &pattern).await?;
    dbg!(&response2);

    Ok(())
}
