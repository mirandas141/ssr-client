use std::str::FromStr;

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

    pub fn get_targets(&self) -> Vec<Environment> {
        if self.target_environment.is_empty() {
            return vec![
                Environment::Dev,
                Environment::Qa,
                Environment::Uat,
                Environment::Prod,
            ];
        }
        let mut results: Vec<Environment> = Vec::new();
        for target in &self.target_environment {
            results.push(*target);
        }
        results
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: &str = match self {
            Environment::Dev => "dev",
            Environment::Qa => "qa",
            Environment::Uat => "uat",
            Environment::Prod => "prod",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "dev" {
            return Ok(Environment::Dev);
        }
        if s == "qa" {
            return Ok(Environment::Qa);
        }
        if s == "uat" {
            return Ok(Environment::Uat);
        }
        if s == "prod" {
            return Ok(Environment::Prod);
        }
        Err(Error::InvalidEnvironmentTarget(s.into()))
    }
}

#[cfg(test)]
mod cli_tests {
    use super::{Environment::*, *};
    use rstest::*;
    use std::error::Error;

    type Result<T> = core::result::Result<T, Box<dyn Error>>;

    #[test]
    fn should_have_default_url_with_all_environment_targets() -> Result<()> {
        let result = Cli::try_parse_from(vec!["app"].iter())?;

        assert_eq!("https://ssr.xenial.com", result.url);
        Ok(())
    }

    #[test]
    fn should_have_by_default_all_environment_targets() {
        let result = Cli::try_parse_from(vec!["app"].iter()).expect("to parse cli with defaults");

        assert_eq!(vec![Dev, Qa, Uat, Prod], result.get_targets());
    }

    #[rstest]
    #[case("dev", vec![Dev])]
    #[case("dev,qa", vec![Dev, Qa])]
    #[case("dev,qa,uat", vec![Dev, Qa, Uat])]
    #[case("dev,qa,uat,prod", vec![Dev, Qa, Uat, Prod])]
    #[case("dev,qa,prod", vec![Dev, Qa, Prod])]
    #[case("dev,uat,prod", vec![Dev, Uat, Prod])]
    #[case("uat,prod", vec![Uat, Prod])]
    fn should_return_target_environments_passed(
        #[case] params: &str,
        #[case] expected: Vec<Environment>,
    ) {
        let result = Cli::try_parse_from(vec!["app", "-e", params].iter())
            .expect("environment parameters to parse");

        assert_eq!(expected, result.get_targets());
    }

    #[rstest]
    #[case(Dev, "dev")]
    #[case(Qa, "qa")]
    #[case(Uat, "uat")]
    #[case(Prod, "prod")]
    fn should_convert_environment_to_string(#[case] input: Environment, #[case] expected: String) {
        assert_eq!(input.to_string(), expected);
    }

    #[rstest]
    #[case("dev", Dev)]
    #[case("qa", Qa)]
    #[case("uat", Uat)]
    #[case("prod", Prod)]
    fn should_convert_string_to_environment_enum(
        #[case] input: String,
        #[case] expected: Environment,
    ) {
        assert_eq!(input.parse::<Environment>().unwrap(), expected);
    }
}
