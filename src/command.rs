use serde::{Deserialize, Serialize};
use std::str::{FromStr, Utf8Error};
use structopt::StructOpt;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ChainType {
    Bifrost,
    EOS,
}

impl FromStr for ChainType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bifrost" | "bifrost" | "BIFROST" => Ok(Self::Bifrost),
            "EOS" | "Eos" | "eos" => Ok(Self::EOS),
            _ => Err("Invalid chain type, please input EOS or Bifrost.".to_owned())
        }
    }
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(about = "Bridge command for blockchain interoperability.")]
pub enum BridgeCmd {
    Get {
        #[structopt(short)]
        chain_type: ChainType,
        #[structopt(short)]
        account: String,
        #[structopt(short)]
        url: String,
    },
    SendTransaction {
        #[structopt(short)]
        chain_type: ChainType,
    },
}