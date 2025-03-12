use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub fn consolidate_targets(tasks: Vec<(String, Vec<SsrRecord>)>) -> Vec<SsrResult> {
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

    results.into_values().collect()
}
