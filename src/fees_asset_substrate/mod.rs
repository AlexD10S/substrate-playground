pub mod calls;
pub mod utils;

use subxt::{OnlineClient, SubstrateConfig, utils::{AccountId32, MultiAddress}};
use subxt_signer::sr25519::dev::{self};
use clap::Parser as ClapParser;

use utils::connect;
use calls::{
    create_asset_call, create_pool_with_native_call, mint_token_call,
    provide_liquidity_to_token_native_pool_call, set_asset_metadata_call,
    sign_and_send_batch_calls, sign_and_send_transfer, Call,
};

#[derive(Debug, ClapParser)]
pub struct Opts {}


pub async fn run(_opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let api = connect().await;

    //Create the Token
    let asset_id: u32 = 1;
    //Prepare the setup for the transfer: create asset, pool, add liquidity...
    prepare_setup(api.clone(), asset_id).await;

    std::thread::sleep(std::time::Duration::from_secs(2));

    let dest: MultiAddress<AccountId32, u32> = dev::bob().public_key().into();
    // Sign and send transfer paying fees with ASSET created
    let result = sign_and_send_transfer(api.clone(), dest, 100, asset_id).await;
    println!("{:?}", result);

    //query_assets(api.clone()).await?;
    Ok(())
}

async fn prepare_setup(api: OnlineClient<SubstrateConfig>, asset_id: u32) {
    let alice: MultiAddress<AccountId32, u32> = dev::alice().public_key().into();
    let address: AccountId32 = dev::alice().public_key().into();

    // Initialise an empty call buffer
    let mut call_buffer: Vec<Call> = Vec::<Call>::new();
    call_buffer.push(create_asset_call(asset_id, alice.clone(), 1).unwrap());

    //Set Token Metadata
    const NAME: &str = "Asset1";
    const SYMBOL: &str = "A1";
    call_buffer.push(
        set_asset_metadata_call(
            asset_id,
            NAME.as_bytes().to_vec(),
            SYMBOL.as_bytes().to_vec(),
            0,
        )
        .unwrap(),
    );

    //Mint token
    const AMOUNT_TO_MINT: u128 = 200000;
    call_buffer.push(mint_token_call(asset_id, alice.clone(), AMOUNT_TO_MINT).unwrap());

    //Create Pool
    call_buffer.push(create_pool_with_native_call(asset_id).unwrap());

    //Provide Liquidity to the pool
    call_buffer.push(
        provide_liquidity_to_token_native_pool_call(
            asset_id,
            100000000000000,
            10000,
            0,
            0,
            address,
        )
        .unwrap(),
    );

    // Sign and send batch_call to the network
    if let Err(subxt::Error::Runtime(dispatch_err)) =
        sign_and_send_batch_calls(api, call_buffer).await
    {
        eprintln!("Could not dispatch the call: {}", dispatch_err);
    }
}