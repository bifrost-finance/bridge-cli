
use sp_core::crypto::Ss58Codec;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::AccountId32;
use subxt::{
    PairSigner, DefaultNodeRuntime as BifrostRuntime,
};

mod bifrost_rpc;
mod command;
mod eos_rpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://10.115.27.96:9990";
    let who = "gXCcrjjFX3RPyhHYgwZDmw8oe4JFpd5anko3nTY8VrmnJpe"; // Alice
    let who = AccountId32::from_ss58check(&who).map_err(|_| "Invalid Address".to_owned())?;
    let asset_id = 6;
    let s = bifrost_rpc::assets::get_asset_by_account(url, &who, asset_id).await?;
    println!("alice asset info: {:?}", s);

    println!("-------------------------");
    let seed = "//Alice";
    let from = Pair::from_string(seed, None).map_err(|_| "Invalid seed!".to_owned())?;
    let from = PairSigner::<BifrostRuntime, Pair>::new(from);

    let to = b"jim".to_vec();
    let amount = 10u128;
    let memo = b"hello".to_vec();
    
    let block = bifrost_rpc::bridge_eos::cross_to_eos(url, &from, to, amount, memo).await?;
    dbg!(block);

    Ok(())
}
