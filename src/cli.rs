use crate::error::Error;
use clap::{Parser, ValueEnum};

const URL: &str = "https://ssr.xenial.com";

#[derive(Debug, Parser)]
#[command(version)]
#[command(about)]
#[command(next_line_help = false)]
#[command(
    after_help = "Name, description, and key returned by the ssr service will be evaluated for `filter` as a substring. If ommitted then all returned values from the `url` will be parsed and returned

Ommitting `TARGET_ENVIRONMENT` will result in all target environments being retrieved and process

`url` should not contain the target environment option as that will be added at runtime"
)]
pub struct Cli {
    /// Environment to grab values for
    #[arg(short = 'e', long = "env")]
    #[arg(num_args = 0..4)]
    #[arg(value_delimiter = ',')]
    //#[arg(verbatim_doc_comment)]
    pub target_environment: Vec<Environment>,

    /// Url to retrieve ssr entries from
    #[arg(short, long)]
    #[arg(default_value = URL)]
    pub url: String,

    /// String to filter results by
    pub filter: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Dev,
    Qa,
    Uat,
    Prod,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    pub fn get_targets(&self) -> Vec<(String, String)> {
        if self.target_environment.len() == 0 {
            return vec![
                ("env".into(), Environment::Dev.to_string()),
                ("env".into(), Environment::Qa.to_string()),
                ("env".into(), Environment::Uat.to_string()),
                ("env".into(), Environment::Prod.to_string()),
            ];
        }
        let mut results: Vec<(String, String)> = Vec::new();
        for target in &self.target_environment {
            results.push(("env".into(), target.to_string()));
        }
        results
    }
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Dev => String::from("dev"),
            Environment::Qa => String::from("qa"),
            Environment::Uat => String::from("uat"),
            Environment::Prod => String::from("prod"),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "dev" {
            return Ok(Environment::Dev);
        }
        if value == "qa" {
            return Ok(Environment::Qa);
        }
        if value == "uat" {
            return Ok(Environment::Uat);
        }
        if value == "prod" {
            return Ok(Environment::Prod);
        }
        Err(Error::UnableToCloneClient)
    }
}
