mod fees_asset_substrate;
mod fees_asset_asset_hub;
mod encode_value_rococo;
mod calculate_sovereign_account;

use clap::Parser as ClapParser;

///Different Scripts
#[derive(Debug, ClapParser)]
enum Command {
    FeesAssetSubstrate(fees_asset_substrate::Opts),
    FeesAssetAssetHub(fees_asset_asset_hub::Opts),
    EncodeValueRococo(encode_value_rococo::Opts),
    CalculateSovereignAccount(calculate_sovereign_account::Opts),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::parse();
    match args {
        Command::FeesAssetSubstrate(opts) => fees_asset_substrate::run(opts).await,
        Command::FeesAssetAssetHub(opts) => fees_asset_asset_hub::run(opts).await,
        Command::EncodeValueRococo(opts) => encode_value_rococo::run(opts).await,
        Command::CalculateSovereignAccount(opts) => calculate_sovereign_account::run(opts).await,
    }
}
