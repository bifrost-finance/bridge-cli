use serde::{Deserialize, Serialize};
use std::str::{FromStr, Utf8Error};
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
pub enum BifrostCmd {
    Get {
        url: String,
        account: String,
    },
    PushTransaction {
        url: String,
        private_key: String,
        amount: u32,
    }
}

#[derive(StructOpt, Clone, Debug)]
pub enum EOSCmd {
    Get {
        url: String,
        account: String,
    },
    PushTransaction {
        url: String,
        private_key: String,
        amount: u32,
    }
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(about = "Bridge command for blockchain interoperability.")]
pub enum BridgeCmd {
    EOS(EOSCmd),
    Bifrost(BifrostCmd),
}