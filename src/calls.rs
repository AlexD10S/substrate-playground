use subxt::utils::{AccountId32, MultiAddress};
use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::dev::{self};
use subxt::config::substrate::{AssetTip, SubstrateExtrinsicParamsBuilder as Params};

#[subxt::subxt(runtime_metadata_path = "src/metadata/local_metadata.scale")]
pub mod local {}

// #[subxt::subxt(runtime_metadata_path = "src/metadata/westend_metadata.scale")]
// pub mod westend {}

// pub type Call = local::runtime_types::kitchensink_runtime::RuntimeCall;
pub type Call = local::runtime_types::asset_hub_westend_runtime::RuntimeCall;
type AssetsCall = local::runtime_types::pallet_assets::pallet::Call;
type AssetConversionCall = local::runtime_types::pallet_asset_conversion::pallet::Call;
// type NativeOrAssetId = local::runtime_types::pallet_asset_conversion::types::NativeOrAssetId<u32>;
type MultiLocation = local::runtime_types::staging_xcm::v3::multilocation::MultiLocation;
use local::runtime_types::staging_xcm::v3::junctions::Junctions::{Here, X2};
use local::runtime_types::staging_xcm::v3::junction::Junction::{PalletInstance, GeneralIndex};


// Create an asset call
pub fn create_asset_call(
    asset_id: u32,
    // admin: MultiAddress<AccountId32, u32>,
    admin: MultiAddress<AccountId32, ()>,
    min_balance: u128,
) -> Result<Call, Box<dyn std::error::Error>> {
    let call = Call::Assets(AssetsCall::create {
        id: asset_id,
        admin: admin,
        min_balance: min_balance,
    });

    Ok(call)
}

// Set token metadats
pub fn set_asset_metadata_call(
    asset_id: u32,
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
) -> Result<Call, Box<dyn std::error::Error>> {
    let call = Call::Assets(AssetsCall::set_metadata {
        id: asset_id,
        name: name,
        symbol: symbol,
        decimals: decimals,
    });

    Ok(call)
}

// Mint token
pub fn mint_token_call(
    asset_id: u32,
    // beneficiary: MultiAddress<AccountId32, u32>,
    beneficiary: MultiAddress<AccountId32, ()>,
    amount: u128,
) -> Result<Call, Box<dyn std::error::Error>> {
    let call = Call::Assets(AssetsCall::mint {
        id: asset_id,
        beneficiary: beneficiary,
        amount: amount,
    });

    Ok(call)
}

// Create pool
pub fn create_pool_with_native_call(asset_id: u32) -> Result<Call, Box<dyn std::error::Error>> {
    let call = Call::AssetConversion(AssetConversionCall::create_pool {
        asset1: MultiLocation { parents: 1, interior: Here }, //Native asset which has a MultiLocation represented like that
        asset2: MultiLocation { parents: 0, interior: X2(PalletInstance(50), GeneralIndex(asset_id.into())) } , //The PalletInstance of 50 represents the Assets pallet on AssetHub and the GeneralIndex is the u32 AssetId of the asset.
    });

    Ok(call)
}

// Create pool
pub fn provide_liquidity_to_token_native_pool_call(
    asset_id: u32,
    amount1_desired: u128,
    amount2_desired: u128,
    amount1_min: u128,
    amount2_min: u128,
    mint_to: AccountId32,
) -> Result<Call, Box<dyn std::error::Error>> {
    let call = Call::AssetConversion(AssetConversionCall::add_liquidity {
        asset1: MultiLocation { parents: 1, interior: Here },
        asset2: MultiLocation { parents: 0, interior: X2(PalletInstance(50), GeneralIndex(asset_id.into())) },
        amount1_desired: amount1_desired,
        amount2_desired: amount2_desired,
        amount1_min: amount1_min,
        amount2_min: amount2_min,
        mint_to: mint_to.into(),
    });

    Ok(call)
}

//
// Sign and send the passed call
//
pub async fn sign_and_send_batch_calls(
    api: OnlineClient<SubstrateConfig>,
    calls: Vec<Call>,
) -> Result<(), subxt::Error> {
    let alice_pair_signer = dev::alice();
    let tx = local::tx().utility().batch_all(calls);
    api.tx()
        .sign_and_submit_then_watch_default(&tx, &alice_pair_signer)
        .await?
        .wait_for_in_block()
        .await?
        .wait_for_success()
        .await?;

    Ok(())
}

//
// Sign and send the passed call
//
pub async fn sign_and_send_transfer(
    api: OnlineClient<SubstrateConfig>,
    // dest: MultiAddress<AccountId32, (u32)>,
    dest: MultiAddress<AccountId32, ()>,
    amount: u128,
    asset_id: u32
) -> Result<(), subxt::Error> {
    let alice_pair_signer = dev::alice();
    let balance_transfer_tx = local::tx().balances().transfer(dest, amount);

    let tx_params = Params::new()
    .tip(AssetTip::new(0).of_asset(asset_id));

    let hash = api.tx()
        .sign_and_submit_then_watch(&balance_transfer_tx, &alice_pair_signer, tx_params)
        .await?
        .wait_for_finalized_success()
        .await?
        .has::<local::balances::events::Transfer>()?;
    println!("Balance transfer extrinsic submitted with hash : {hash}");
    Ok(())
}
