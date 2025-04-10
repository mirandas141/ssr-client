use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Environment;

pub struct Ssr {
    records: Vec<SsrRecords>,
    pattern: Option<String>,
}

impl Ssr {
    pub fn new(capacity: usize) -> Self {
        Ssr {
            records: Vec::with_capacity(capacity),
            pattern: None,
        }
    }

    pub fn set_pattern(self, pattern: Option<String>) -> Self {
        let pattern = pattern.map(|val| val.to_lowercase());
        Ssr {
            records: self.records,
            pattern,
        }
    }

    pub fn add_records(&mut self, target: Environment, records: Vec<SsrRecord>) {
        self.records.push(SsrRecords::new(target, records));
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn consolidate(self) -> Vec<SsrResult> {
        let mut results: HashMap<String, SsrResult> = HashMap::new();

        for target in self.records {
            for record in target
                .records
                .into_iter()
                .filter(|r| r.matches_pattern(&self.pattern))
            {
                if let Some(r) = results.get_mut(&record.key.clone()) {
                    r.update_url(target.target, record.url);
                } else {
                    let mut r = SsrResult::new(&record.name, &record.description, &record.key);
                    r.update_url(target.target, record.url);
                    results.insert(record.key, r);
                }
            }
        }

        results.into_values().collect()
    }
}

struct SsrRecords {
    target: Environment,
    records: Vec<SsrRecord>,
}

impl SsrRecords {
    fn new(target: Environment, records: Vec<SsrRecord>) -> Self {
        SsrRecords {
            target: target.into(),
            records,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SsrRecord {
    pub name: String,
    pub description: String,
    pub key: String,
    pub url: String,
}

impl SsrRecord {
    fn matches_pattern(&self, pattern: &Option<String>) -> bool {
        match &pattern {
            Some(value) => {
                self.name.to_lowercase().contains(value)
                    || self.description.to_lowercase().contains(value)
                    || self.key.to_lowercase().contains(value)
            }
            None => true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SsrResult {
    name: String,
    description: String,
    key: String,
    url: Vec<(Environment, Option<String>)>,
}

impl SsrResult {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        let mut targets = Vec::new();
        targets.push((Environment::Dev, None));
        targets.push((Environment::Qa, None));
        targets.push((Environment::Uat, None));
        targets.push((Environment::Prod, None));
        SsrResult {
            name: name.into(),
            description: description.into(),
            key: key.into(),
            url: targets,
        }
    }
}

impl Environment {
    fn to_usize(&self) -> usize {
        match self {
            Environment::Dev => 0,
            Environment::Qa => 1,
            Environment::Uat => 2,
            Environment::Prod => 3,
        }
    }
}

impl SsrResult {
    pub fn update_url(&mut self, target: Environment, value: impl Into<String>) {
        self.url[target.to_usize()] = (target, Some(value.into()))
    }
}

impl std::fmt::Display for SsrResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})\n{}\n", &self.name, &self.key, &self.description)?;
        for target in self.url.iter() {
            if let Some(value) = &target.1 {
                write!(f, "{} \t {}\n", target.0, value)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_shared {
    use super::*;

    impl SsrRecord {
        pub fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            key: impl Into<String>,
            url: impl Into<String>,
        ) -> Self {
            SsrRecord {
                name: name.into(),
                description: description.into(),
                key: key.into(),
                url: url.into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ssr::cli::Environment;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("SsR nAmE", "desc", "key", "https://somewhere", Some("nAmE".into()), 1)]
    #[case("ssr name", "desc", "key", "https://somewhere", Some("name ".into()), 0)]
    #[case("ssr name", "DesC", "key", "https://somewhere", Some("dEs".into()), 1)]
    #[case("ssr name", "desc", "kEy", "https://somewhere", Some("Ke".into()), 1)]
    #[case("ssr name", "desc", "KeY", "https://somewhere", Some("kEy".into()), 1)]
    #[case("ssr name", "desc", "key", "https://somewhere", Some("somewhere".into()), 0)]
    #[case("ssr name", "desc", "key", "https://somewhere", None, 1)]
    fn should_only_include_records_containing_pattern_during_consolidation(
        #[case] name: String,
        #[case] description: String,
        #[case] key: String,
        #[case] url: String,
        #[case] pattern: Option<String>,
        #[case] expected_count: usize,
    ) {
        let mut records = Vec::new();
        let record = SsrRecord::new(name, description, key, url);
        records.push(record);
        let mut sut = Ssr::new(1).set_pattern(pattern);
        sut.add_records(Environment::Dev, records);

        assert_eq!(sut.consolidate().len(), expected_count);
    }

    #[test]
    fn should_group_urls_together_under_same_key() {
        let mut records = Vec::new();
        records.push(SsrRecord::new("name", "description", "key", "https://url1"));
        let mut sut = Ssr::new(2).set_pattern(None);
        sut.add_records(Environment::Dev, records);
        let mut records = Vec::new();
        records.push(SsrRecord::new("name", "description", "key", "https://url2"));
        sut.add_records(Environment::Uat, records);

        let actual = sut.consolidate();

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].url.len(), 4);
        assert_eq!(
            actual[0].url[Environment::Dev.to_usize()].1,
            Some("https://url1".into())
        );
        assert_eq!(actual[0].url[Environment::Qa.to_usize()].1, None);
        assert_eq!(
            actual[0].url[Environment::Uat.to_usize()].1,
            Some("https://url2".into())
        );
        assert_eq!(actual[0].url[Environment::Prod.to_usize()].1, None);
    }

    #[test]
    fn will_overwrite_earlier_url_for_same_key_and_target() {
        let mut records = Vec::new();
        records.push(SsrRecord::new("name", "description", "key", "https://url1"));
        records.push(SsrRecord::new("name", "description", "key", "https://url2"));
        let mut sut = Ssr::new(2).set_pattern(None);
        sut.add_records(Environment::Dev, records);

        let actual = sut.consolidate();

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].url.len(), 4);
        assert_eq!(
            actual[0].url[Environment::Dev.to_usize()].1,
            Some("https://url2".into())
        );
        assert_eq!(actual[0].url[Environment::Qa.to_usize()].1, None);
        assert_eq!(actual[0].url[Environment::Uat.to_usize()].1, None);
        assert_eq!(actual[0].url[Environment::Prod.to_usize()].1, None);
    }
}
