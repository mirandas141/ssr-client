mod error;
mod ssr;

use crate::error::Result;
use crate::ssr::Cli;
use crate::ssr::SsrRetriever;

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
