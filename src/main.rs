mod cli;
mod error;

use std::collections::{HashMap, HashSet};

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

fn env_parse(targets: &Vec<Environment>) -> Vec<(&str, &str)> {
    if targets.len() == 1 {
        return vec![
            ("env", "dev"),
            ("env", "qa"),
            ("env", "uat"),
            ("env", "prod"),
        ];
    }
    let mut results: Vec<(&str, &str)> = Vec::new();
    for target in targets {
        results.push(match target {
            Environment::Dev => ("env", "dev"),
            Environment::Qa => ("env", "qa"),
            Environment::Uat => ("env", "uat"),
            Environment::Prod => ("env", "prod"),
        });
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
            &_ => (),
        }
    }
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
    let targets = env_parse(&cli.target_environment);
    let mut tasks = Vec::with_capacity(targets.len());

    for target in targets {
        tasks.push((target.1, get_records(&client, &target, &pattern).await?));
    }
    dbg!(&tasks);

    let mut results: HashMap<String, SsrResult> = HashMap::new();

    for target in tasks {
        for result in target.1 {
            if let Some(r) = results.get_mut(&result.key.clone()) {
                r.url.update(target.0, result.url);
                /*
                match target.0 {
                    "dev" => r.url.dev = Some(result.url),
                    "qa" => r.url.qa = Some(result.url),
                    "uat" => r.url.uat = Some(result.url),
                    "prod" => r.url.prod = Some(result.url),
                    &_ => (),
                }
                */
            } else {
                let mut r = SsrResult::new(&result.name, &result.description, &result.key);
                r.url.update(target.0, result.url);
                results.insert(result.key, r);
            }
        }
    }

    dbg!(results);

    //let result: Vec<Ssr> = tasks.1.into_iter().flatten().collect();

    //println!("results: {:#?}", &result);
    Ok(())
}
