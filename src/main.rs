mod fees_asset_substrate;
mod fees_asset_asset_hub;

use clap::Parser as ClapParser;

///Different Scripts
#[derive(Debug, ClapParser)]
enum Command {
    FeesAssetSubstrate(fees_asset_substrate::Opts),
    FeesAssetAssetHub(fees_asset_asset_hub::Opts),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::parse();
    match args {
        Command::FeesAssetSubstrate(opts) => fees_asset_substrate::run(opts).await,
        Command::FeesAssetAssetHub(opts) => fees_asset_asset_hub::run(opts).await,
    }
}
