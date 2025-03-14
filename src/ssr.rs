use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Ssr {
    records: Vec<SsrRecords>,
}

impl Ssr {
    pub fn new(capacity: usize) -> Self {
        Ssr {
            records: Vec::with_capacity(capacity),
        }
    }

    pub fn add_records(&mut self, target: impl Into<String>, records: Vec<SsrRecord>) {
        self.records.push(SsrRecords::new(target, records));
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn consolidate(self) -> Vec<SsrResult> {
        let mut results: HashMap<String, SsrResult> = HashMap::new();
        let records = self.records;

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
}

struct SsrRecords {
    target: String,
    records: Vec<SsrRecord>,
}

impl SsrRecords {
    pub fn new(target: impl Into<String>, records: Vec<SsrRecord>) -> Self {
        SsrRecords {
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
