use clap::{Parser, ValueEnum};

const URL: &str = "https://ssr.xenial.com";

#[derive(Debug, Parser)]
#[command(version)]
#[command(about)]
pub struct Cli {
    #[arg(short = 'e', long = "env")]
    #[arg(num_args = 0..4)]
    #[arg(value_delimiter = ',')]
    pub target_environment: Vec<Environment>,

    #[arg(short, long)]
    #[arg(default_value = URL)]
    pub url: String,

    #[arg(short, long)]
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
        Err(crate::error::Error::custom("unknown environment"))
    }
}
