mod cli;
mod error;

use std::collections::HashMap;

use crate::cli::Cli;
use crate::error::Result;
use cli::Environment;
use error::Error;
use reqwest::Client;
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
    url: SsrUrl,
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
            url: SsrUrl::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct SsrUrl {
    dev: Option<String>,
    qa: Option<String>,
    uat: Option<String>,
    prod: Option<String>,
}

impl SsrUrl {
    pub fn new() -> Self {
        SsrUrl {
            dev: None,
            qa: None,
            uat: None,
            prod: None,
        }
    }

    pub fn update(&mut self, target: &str, value: impl Into<String>) {
        let value = value.into();
        match target {
            "dev" => self.dev = Some(value),
            "qa" => self.qa = Some(value),
            "uat" => self.uat = Some(value),
            "prod" => self.prod = Some(value),
            &_ => unreachable!("Unknown target environment"),
        }
    }
}

async fn get_records(
    client: &reqwest::RequestBuilder,
    target: &(String, String),
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
    let targets = env_parse(&cli.target_environment);
    let mut tasks = Vec::with_capacity(targets.len());

    for target in targets {
        tasks.push((
            target.1.clone(),
            get_records(&client, &target, &pattern).await?,
        ));
    }

    let mut results: HashMap<String, SsrResult> = HashMap::new();

    for target in tasks {
        for result in target.1 {
            if let Some(r) = results.get_mut(&result.key.clone()) {
                r.url.update(&target.0, result.url);
            } else {
                let mut r = SsrResult::new(&result.name, &result.description, &result.key);
                r.url.update(&target.0, result.url);
                results.insert(result.key, r);
            }
        }
    }

    dbg!(results);
    Ok(())
}
