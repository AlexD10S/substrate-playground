use clap::Parser as ClapParser;

use scale::Encode;
use subxt::utils::AccountId32;
use sp_core::sr25519::Pair;
use sp_core::crypto::{Ss58AddressFormatRegistry, Ss58Codec};
use sp_runtime::MultiSigner;

#[subxt::subxt(runtime_metadata_path = "src/metadata/rococo_metadata.scale")]
pub mod rococo {}


use rococo::runtime_types::polkadot_parachain::primitives::Id;

#[derive(Debug, ClapParser)]
pub struct Opts {
     /// Parachain id we want to get the sovereign account.
     pub para_id: u32,
}


pub async fn run(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let sovereign_account = calculate_sovereign_account::<Pair>(opts.para_id.clone()).unwrap();
    let _sovereign_account_accountId: AccountId32 = sovereign_account.parse().unwrap();
    println!("Sovereing Account: {:?}", sovereign_account);

    Ok(())
}

pub fn calculate_sovereign_account<Pair>(
    para_id: u32,
) -> Result<String, Box<dyn std::error::Error>>
where
    Pair: sp_core::Pair,
    Pair::Public: Into<MultiSigner>,
{
    let id = Id(para_id);
    let prefix = hex::encode("para");
    let encoded_id = hex::encode(id.encode());
    let encoded_key = "0x".to_owned() + &prefix + &encoded_id;
    let public_str = format!("{:0<width$}", encoded_key, width = 64 + 2);

    let public = array_bytes::hex2bytes(&public_str).expect("Failed to convert hex to bytes");
    let public_key = Pair::Public::try_from(&public)
        .map_err(|_| "Failed to construct public key from given hex")?;
    let to_parse =
        public_key.to_ss58check_with_version(Ss58AddressFormatRegistry::SubstrateAccount.into());
    Ok(to_parse)
}