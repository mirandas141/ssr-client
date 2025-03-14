mod cli;
mod error;
mod retriever;
mod ssr;

use crate::cli::Cli;
use crate::error::Result;
use retriever::SsrRetriever;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let mut targets = cli.get_targets();
    let pattern = cli.filter.map(|val| val.to_lowercase());
    let records = SsrRetriever::new(&cli.url)
        .add_targets(&mut targets)
        .get(pattern)?;

    let results = ssr::consolidate_targets(records);
    println!("{:#?}", results);

    Ok(())
}
