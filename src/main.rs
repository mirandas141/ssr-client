mod error;
mod ssr;

use std::process::exit;

use crate::error::Result;
use crate::ssr::{Cli, SsrRetriever};

fn main() {
    let cli = Cli::parse_args();
    match process(cli) {
        Ok(records) => println!("{:#?}", records),
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }
}

fn process(cli: Cli) -> Result<()> {
    let records = SsrRetriever::new(&cli.url)
        .add_targets(&mut cli.get_targets())
        .get()?
        .set_pattern(cli.filter)
        .consolidate();

    for record in records.iter() {
        println!("{}", record);
    }

    std::process::exit(0)
}
