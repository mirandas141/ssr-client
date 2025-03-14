use crate::error::{Error, Result};
use crate::ssr::{Ssr, SsrRecord};
use reqwest::blocking::{Client, RequestBuilder};

pub struct SsrRetriever {
    client: RequestBuilder,
    targets: Vec<(String, String)>,
}

impl SsrRetriever {
    pub fn new(url: impl Into<String>) -> Self {
        SsrRetriever {
            client: Client::new().get(url.into()),
            targets: Vec::new(),
        }
    }

    pub fn add_targets(mut self, targets: &mut Vec<(String, String)>) -> Self {
        self.targets.append(targets);
        self
    }

    pub fn get(&self, pattern: Option<String>) -> Result<Ssr> {
        retrieve_from(&self.client, &self.targets, pattern)
    }
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

fn retrieve_from(
    client: &reqwest::blocking::RequestBuilder,
    targets: &Vec<(String, String)>,
    pattern: Option<String>,
) -> Result<Ssr> {
    let mut records = Ssr::new(targets.len());
    let pattern = pattern.map(|val| val.to_lowercase());

    for target in targets {
        let ssr_result = get_records(&client, &target, &pattern);
        match ssr_result {
            Ok(result) => records.add_records(target.1.clone(), result),
            Err(Error::Reqwest(e)) => eprintln!("{}", e.to_string()),
            Err(Error::UnableToCloneClient) => eprintln!("Unable to process request"),
            Err(_) => eprintln!("Failed to retrieve ssr records from endpoint!"),
        }
    }

    if records.is_empty() {
        return Err(Error::NoRecordsToProcess);
    }
    Ok(records)
}
