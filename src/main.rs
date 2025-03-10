mod cli {
    use clap::{Parser, ValueEnum};

    #[derive(Debug, Parser)]
    #[command(version)]
    #[command(about)]
    pub struct Cli {
        #[arg(short = 'e', long = "env")]
        target_environment: Environment,
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
}

pub mod error {
    use derive_more::From;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, From)]
    pub enum Error {
        Custom(String),
        #[from]
        Reqwest(reqwest::Error),
    }

    impl Error {
        pub fn custom(val: impl core::fmt::Display) -> Self {
            Self::Custom(val.to_string())
        }
    }

    impl From<&str> for Error {
        fn from(value: &str) -> Self {
            Self::Custom(value.to_string())
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(fmt, "{}", &self)
        }
    }

    impl core::error::Error for Error {}
}

use crate::cli::Cli;
use crate::error::Result;
use reqwest::Client;

const URL: &str = "https://ssr.xenial.com";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    println!("Args: {:#?}", cli);

    let client = Client::new();
    let response = client.get(URL).send().await?.text().await?;

    println!("Response: {:#?}", response);

    Ok(())
}
