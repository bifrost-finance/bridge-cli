
use sp_core::crypto::Ss58Codec;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::AccountId32;
use subxt::{
    PairSigner, DefaultNodeRuntime as BifrostRuntime,
};
use structopt::StructOpt;

mod bifrost_rpc;
mod command;
mod eos_rpc;

use crate::command::{BridgeCmd, EOSCmd, BifrostCmd};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = command::BridgeCmd::from_args();
    println!("{:?}", args);

    match args {
        BridgeCmd::EOS(eos_args) => {
            println!("{:?}", eos_args);
            match eos_args {
                EOSCmd::Get { ref url, ref account } => {
                    let block = eos_rpc::get_account(url, account).await;
                    println!("block num: {:?}", block);
                }
                EOSCmd::PushTransaction { ref url, private_key, amount } => {
                    let block = eos_rpc::get_block(url, &amount.to_string()).await;
                }
            }
        }
        BridgeCmd::Bifrost(bifrost_args) => {
            println!("{:?}", bifrost_args);
            match bifrost_args {
                BifrostCmd::Get { ref url, account , asset_id } => {
                    let who = AccountId32::from_ss58check(&account).map_err(|_| "Invalid Address".to_owned())?;
                    let user_asset = bifrost_rpc::assets::get_asset_by_account(url, &who, asset_id).await?;
                    println!("user asset: {:?}", user_asset);
                }
                BifrostCmd::PushTransaction { ref url, ref from, to, amount, memo } => {
                    let from = Pair::from_string(from, None).map_err(|_| "Invalid seed!".to_owned())?;
                    let from = PairSigner::<BifrostRuntime, Pair>::new(from);
                    let block = bifrost_rpc::bridge_eos::cross_to_eos(url, &from, to.as_bytes().to_vec(), amount, memo.as_bytes().to_vec()).await?;
                    println!("This transaction happened on block: {:?}", block);
                }
            }
        }
    }

    // let url = "https://jungle3.cryptolions.io:443";
    // let info = eos_rpc::get_info(url).await;
    // let block = eos_rpc::get_block(url, "58241176").await;
    // println!("info: {:?}", info);
    // println!("info: {:?}", block);

    // // let url = "ws://127.0.0.1:9944";
    // let url = "ws://150.109.194.40:9944";
    // let who = "gXCcrjjFX3RPyhHYgwZDmw8oe4JFpd5anko3nTY8VrmnJpe"; // Alice
    // let who = AccountId32::from_ss58check(&who).map_err(|_| "Invalid Address".to_owned())?;
    // let asset_id = 6;
    // let s = bifrost_rpc::assets::get_asset_by_account(url, &who, asset_id).await?;
    // println!("alice asset info: {:?}", s);

    println!("-------------------------");
    // let seed = "//Alice";
    // let from = Pair::from_string(seed, None).map_err(|_| "Invalid seed!".to_owned())?;
    // let from = PairSigner::<BifrostRuntime, Pair>::new(from);

    // let to = b"jim".to_vec();
    // let amount = 10_000_000_000_000u128;
    // let memo = b"hello".to_vec();
    
    // let block = bifrost_rpc::bridge_eos::cross_to_eos(url, &from, to, amount, memo).await?;
    // dbg!(block);

    Ok(())
}
