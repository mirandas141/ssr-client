mod cli;
mod error;

use std::collections::HashMap;

use crate::cli::Cli;
use crate::error::{Error, Result};
use cli::Environment;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct Ssr {
    name: String,
    description: String,
    key: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SsrResult {
    name: String,
    description: String,
    key: String,
    url: HashMap<String, String>,
}

impl SsrResult {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        SsrResult {
            name: name.into(),
            description: description.into(),
            key: key.into(),
            url: HashMap::new(),
        }
    }
}

impl SsrResult {
    pub fn update_url(&mut self, target: impl Into<String>, value: impl Into<String>) {
        let _ = self.url.insert(target.into(), value.into());
    }
}

fn get_records(
    client: &reqwest::blocking::RequestBuilder,
    target: &(String, String),
    pattern: &Option<String>,
) -> Result<Vec<Ssr>> {
    let result = client
        .try_clone()
        .ok_or_else(|| Error::UnableToCloneClient)?
        .query(&[target])
        .send()?
        .json::<Vec<Ssr>>()?
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

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let client = Client::new().get(&cli.url);
    let pattern = &cli.filter.clone().map(|val| val.to_lowercase());
    let targets = env_parse(&cli.target_environment);
    let mut tasks = Vec::new();

    for target in targets {
        let ssr_result = get_records(&client, &target, pattern);
        match ssr_result {
            Ok(result) => tasks.push((target.1.clone(), result)),
            Err(_) => eprintln!("Failed to retrieve ssr records from endpoint!"),
        }
    }

    if tasks.is_empty() {
        println!("No records found to process");
        return Err(Error::NoRecordsToProcess);
    }

    let mut results: HashMap<String, SsrResult> = HashMap::new();

    for target in tasks {
        for result in target.1 {
            if let Some(r) = results.get_mut(&result.key.clone()) {
                r.update_url(&target.0, result.url);
            } else {
                let mut r = SsrResult::new(&result.name, &result.description, &result.key);
                r.update_url(&target.0, result.url);
                results.insert(result.key, r);
            }
        }
    }

    println!("{:#?}", results);
    Ok(())
}
