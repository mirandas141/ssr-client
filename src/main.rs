mod cli;
mod error;
mod retriever;
mod ssr;

use crate::cli::Cli;
use crate::error::Result;
use reqwest::blocking::Client;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let client = Client::new().get(&cli.url);
    let targets = cli.get_targets();
    let records = retriever::retrieve_from(&client, targets, cli.filter)?;

    let results = ssr::consolidate_targets(records);
    println!("{:#?}", results);

    Ok(())
}
