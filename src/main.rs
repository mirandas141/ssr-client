mod cli;
mod error;
mod ssr;

use crate::cli::Cli;
use crate::error::{Error, Result};
use crate::ssr::SsrRecord;
use cli::Environment;
use reqwest::blocking::Client;

fn env_parse(targets: &Vec<Environment>) -> Vec<(String, String)> {
    if targets.len() == 0 {
        return vec![
            ("env".into(), Environment::Dev.to_string()),
            ("env".into(), Environment::Qa.to_string()),
            ("env".into(), Environment::Uat.to_string()),
            ("env".into(), Environment::Prod.to_string()),
        ];
    }
    let mut results: Vec<(String, String)> = Vec::new();
    for target in targets {
        results.push(("env".into(), target.to_string()));
    }
    results
}

fn get_records(
    client: &reqwest::blocking::RequestBuilder,
    target: &(String, String),
    pattern: &Option<String>,
) -> Result<Vec<SsrRecord>> {
    let result = client
        .try_clone()
        .ok_or_else(|| Error::UnableToCloneClient)?
        .query(&[target])
        .send()?
        .json::<Vec<SsrRecord>>()?
        .into_iter()
        .filter(|record| match &pattern {
            Some(value) => {
                record.name.to_lowercase().contains(value)
                    || record.description.to_lowercase().contains(value)
                    || record.key.to_lowercase().contains(value)
            }
            None => true,
        })
        .collect::<Vec<SsrRecord>>();
    Ok(result)
}

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let client = Client::new().get(&cli.url);
    let targets = env_parse(&cli.target_environment);
    let records = retrieve_from(&client, targets, cli.filter)?;

    let results = ssr::consolidate_targets(records);
    println!("{:#?}", results);

    Ok(())
}

fn retrieve_from(
    client: &reqwest::blocking::RequestBuilder,
    targets: Vec<(String, String)>,
    pattern: Option<String>,
) -> Result<Vec<(String, Vec<SsrRecord>)>> {
    let mut records = Vec::with_capacity(targets.len());
    let pattern = pattern.map(|val| val.to_lowercase());

    for target in targets {
        let ssr_result = get_records(&client, &target, &pattern);
        match ssr_result {
            Ok(result) => records.push((target.1.clone(), result)),
            Err(_) => eprintln!("Failed to retrieve ssr records from endpoint!"),
        }
    }

    if records.is_empty() {
        return Err(Error::NoRecordsToProcess);
    }
    Ok(records)
}
