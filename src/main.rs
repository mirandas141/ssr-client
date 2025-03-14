mod cli;
mod error;
mod retriever;
mod ssr;

use crate::cli::Cli;
use crate::error::Result;
use retriever::SsrRetriever;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let records = SsrRetriever::new(&cli.url)
        .add_targets(&mut cli.get_targets())
        .get()?
        .set_pattern(cli.filter)
        .consolidate();

    println!("{:#?}", records);

    Ok(())
}
