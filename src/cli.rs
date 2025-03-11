use clap::{Parser, ValueEnum};

const URL: &str = "https://ssr.xenial.com";

#[derive(Debug, Parser)]
#[command(version)]
#[command(about)]
pub struct Cli {
    #[arg(short = 'e', long = "env")]
    #[arg(default_value = "dev")]
    pub target_environment: Environment,

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
