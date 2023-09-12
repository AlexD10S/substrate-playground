use subxt::{
    utils::{AccountId32, MultiAddress},
    OnlineClient, SubstrateConfig,
};
use subxt_playground::calls::{
    create_asset_call, create_pool_with_native_call, mint_token_call,
    provide_liquidity_to_token_native_pool_call, set_asset_metadata_call,
    sign_and_send_batch_calls, sign_and_send_transfer, Call,
};
use subxt_playground::query::query_existential_deposit;
use subxt_signer::sr25519::dev::{self};

pub async fn connect() -> OnlineClient<SubstrateConfig> {
    let uri = "ws://127.0.0.1:9944".to_string();
    let api = OnlineClient::<SubstrateConfig>::from_url(uri)
        .await
        .expect("Connection to Rococo failed");
    api
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = connect().await;

    // let alice: MultiAddress<AccountId32, u32> = dev::alice().public_key().into();
    let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();
    let address: AccountId32 = dev::alice().public_key().into();

    // Initialise an empty call buffer
    let mut call_buffer: Vec<Call> = Vec::<Call>::new();
    //Create the Token
    const ASSET_ID: u32 = 1;
    call_buffer.push(create_asset_call(ASSET_ID, alice.clone(), 1).unwrap());

    //Set Token Metadata
    const NAME: &str = "Asset1";
    const SYMBOL: &str = "A1";
    call_buffer.push(set_asset_metadata_call(ASSET_ID, NAME.as_bytes().to_vec(), SYMBOL.as_bytes().to_vec(), 0).unwrap());

    //Mint token
    const AMOUNT_TO_MINT: u128 = 200000;
    call_buffer.push(mint_token_call(ASSET_ID, alice.clone(), AMOUNT_TO_MINT).unwrap());

    //Create Pool
    call_buffer.push(create_pool_with_native_call(ASSET_ID).unwrap());
    //Provide Liquidity to the pool
    let existential_deposit = query_existential_deposit(api.clone())?;
    println!("ed {:?}", existential_deposit);
    call_buffer.push(
        provide_liquidity_to_token_native_pool_call(
            ASSET_ID,
            existential_deposit,
            10000,
            0,
            0,
            address,
        )
        .unwrap(),
    );

    // Sign and send batch_call to the network
    if let Err(subxt::Error::Runtime(dispatch_err)) = sign_and_send_batch_calls(api.clone(), call_buffer).await  {
        eprintln!("Could not dispatch the call: {}", dispatch_err);
    }

    std::thread::sleep(std::time::Duration::from_secs(2));

    // let dest: MultiAddress<AccountId32, u32> = dev::bob().public_key().into();
    let dest: MultiAddress<AccountId32, ()> = dev::bob().public_key().into();
    // Sign and send transfer paying fees with ASSET created
    let a = sign_and_send_transfer(api.clone(),dest, 100, ASSET_ID).await;
    println!("{:?}", a);
  

    //query_assets(api.clone()).await?;
    Ok(())
}
