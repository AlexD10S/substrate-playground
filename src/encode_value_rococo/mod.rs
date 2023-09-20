use clap::Parser as ClapParser;

use subxt::{OnlineClient, SubstrateConfig};
use subxt::utils::AccountId32;
use sp_core::Encode;

#[subxt::subxt(runtime_metadata_path = "src/metadata/rococo_metadata.scale")]
pub mod rococo {}

type RegistrarCall = rococo::runtime_types::polkadot_runtime_common::paras_registrar::pallet::Call;
use rococo::runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo;
use rococo::runtime_types::polkadot_parachain::primitives::Id as RococoId;

pub async fn connect() -> OnlineClient<SubstrateConfig> {
    let uri = "wss://rococo-rpc.polkadot.io:443".to_string();
    let api = OnlineClient::<SubstrateConfig>::from_url(uri)
        .await
        .expect("Connection to Rococo failed");
    api
}

//
// Check the next free para available in in Rococo
//
// pub async fn query_paras(api: OnlineClient<SubstrateConfig>, para_id: u32) -> Option<ParaInfo<AccountId32, u128>>{
//     let query = rococo::storage().registrar().paras(RococoId(para_id));
//     let result = api
//         .storage()
//         .at_latest()
//         .await
//         .expect("Error with the query1")
//         .fetch(&query)
//         .await
//         .expect("Error with the query");
//     println!("Result: {:?}", result);
// }

#[derive(Debug, ClapParser)]
pub struct Opts {}


pub async fn run(_opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let rococo_api = connect().await;
    //Test is for Trappist, id= 1836
    // TODO: get the raw value before being decoded
    // let ab = query_paras(rococo_api, 1836).await;
    // println!("ab: {:?}", ab);
    let account_hardcoded: AccountId32 = "5EM3YQTLVcQkqhLa4QJ17VxhaLn8osPfF4bNC72JFeDDgdiC".parse().unwrap();
    let para_info = ParaInfo {
        manager: account_hardcoded,
        deposit: 10,
        locked: false,
    };
    let bytes_hex = format!("0x{}", hex::encode(para_info.encode()));
    println!("Encoded Value: {:?}", bytes_hex);

    Ok(())
}