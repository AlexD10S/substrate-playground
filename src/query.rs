use subxt::{OnlineClient, SubstrateConfig};

#[subxt::subxt(runtime_metadata_path = "src/metadata/local_metadata.scale")]
pub mod local {}

// #[subxt::subxt(runtime_metadata_path = "src/metadata/westend_metadata.scale")]
// pub mod westend {}

//
// Query the Existential Deposit
//
pub fn query_existential_deposit(
    api: OnlineClient<SubstrateConfig>,
) -> Result<u128, Box<dyn std::error::Error>> {
    let query = local::constants().balances().existential_deposit();
    let value = api.constants().at(&query)?;
    Ok(value)
}
//
// Query the assets
//
pub async fn query_assets(
    api: OnlineClient<SubstrateConfig>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let query = local::storage().assets().asset(1);
    match api.storage().at_latest().await?.fetch(&query).await? {
        Some(asset) => {
            println!("Asset: {:?}", asset);
            Ok(true)
        }
        _ => Ok(false),
    }
}
