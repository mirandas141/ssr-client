use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Ssr {
    target: String,
    records: Vec<SsrRecord>,
}

impl Ssr {
    pub fn new(target: impl Into<String>, records: Vec<SsrRecord>) -> Self {
        Ssr {
            target: target.into(),
            records,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsrRecord {
    pub name: String,
    pub description: String,
    pub key: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsrResult {
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

pub fn consolidate_targets(records: Vec<Ssr>) -> Vec<SsrResult> {
    let mut results: HashMap<String, SsrResult> = HashMap::new();

    for target in records {
        for result in target.records {
            if let Some(r) = results.get_mut(&result.key.clone()) {
                r.update_url(&target.target, result.url);
            } else {
                let mut r = SsrResult::new(&result.name, &result.description, &result.key);
                r.update_url(&target.target, result.url);
                results.insert(result.key, r);
            }
        }
    }

    results.into_values().collect()
}
