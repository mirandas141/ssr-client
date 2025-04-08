use super::cli::Environment;
use super::ssr::{Ssr, SsrRecord};
use crate::error::{Error, Result};
use reqwest::blocking::{Client, RequestBuilder};

pub struct SsrRetriever {
    client: RequestBuilder,
    targets: Vec<(String, Environment)>,
}

impl SsrRetriever {
    pub fn new(url: impl Into<String>) -> Self {
        SsrRetriever {
            client: Client::new().get(url.into()),
            targets: Vec::new(),
        }
    }

    pub fn add_targets(mut self, targets: &mut [Environment]) -> Self {
        let mut values = targets
            .iter_mut()
            .map(|env| (String::from("env"), env.clone()))
            .collect::<Vec<(String, Environment)>>();
        self.targets.append(&mut values);
        self
    }

    pub fn get(&self) -> Result<Ssr> {
        retrieve_from(&self.client, &self.targets)
    }
}

fn get_records(
    client: &reqwest::blocking::RequestBuilder,
    target: &(String, Environment),
) -> Result<Vec<SsrRecord>> {
    let result = client
        .try_clone()
        .ok_or_else(|| Error::UnableToCloneClient)?
        .query(&[(target.0.clone(), target.1.to_string())])
        .send()?
        .json::<Vec<SsrRecord>>()?;
    Ok(result)
}

fn retrieve_from(
    client: &reqwest::blocking::RequestBuilder,
    targets: &Vec<(String, Environment)>,
) -> Result<Ssr> {
    let mut records = Ssr::new(targets.len());

    for target in targets {
        let ssr_result = get_records(client, target);
        match ssr_result {
            Ok(result) => records.add_records(target.1, result),
            Err(e) => return Err(e),
        }
    }

    if records.is_empty() {
        return Err(Error::NoRecordsToProcess);
    }
    Ok(records)
}
