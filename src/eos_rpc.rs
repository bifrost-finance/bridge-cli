use serde::{Deserialize, Serialize};
use std::error;

const GET_ACCOUNT_API: &'static str = "/v1/chain/get_account";
const GET_INFO_API: &'static str = "/v1/chain/get_info";
const GET_BLOCK_API: &'static str = "/v1/chain/get_block";
const PUSH_TRANSACTION_API: &'static str = "/v1/chain/push_transaction";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GetAccount {
    pub account_name: String,
    pub head_block_num: u32,
    pub head_block_time: String,
    pub last_code_update: String,
    pub created: String,
    pub net_weight: String,
    pub cpu_weight: String,
    pub ram_usage: String,
    pub privileged: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GetAccountParam {
    pub account_name: String,
}

impl GetAccountParam {
    pub fn new(account_name: &str) -> Self {
        Self { account_name: account_name.to_owned() }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetBlockParams {
    block_num_or_id: String,
}

impl GetBlockParams {
    pub fn new(block_num_or_id: &str) -> Self {
        Self { block_num_or_id: block_num_or_id.to_owned() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBlock {
    pub timestamp: String,
    pub producer: String,
    pub confirmed: u16,
    pub previous: String,
    pub transaction_mroot: String,
    pub action_mroot: String,
    pub schedule_version: u16,
    #[serde(skip)]
    pub new_producers: Option<String>,
    #[serde(skip)]
    pub header_extensions: Vec<String>,
    pub producer_signature: String,
    pub transactions: Vec<String>,
    #[serde(skip)]
    pub block_extensions: Vec<String>,
    pub id: String,
    pub block_num: u64,
    pub ref_block_prefix: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GetInfoParams {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetInfo {
    pub server_version: String,
    pub chain_id: String,
    pub head_block_num: u32,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub last_irreversible_block_num: u32,
    pub last_irreversible_block_id: String,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
    pub server_version_string: String,
    pub fork_db_head_block_num: u32,
    pub fork_db_head_block_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PushTransactionParams {
    pub signatures: Vec<String>,
    pub compression: String,
    pub packed_context_free_data: String,
    pub packed_trx: String,
}

impl PushTransactionParams {
    pub fn new(
        signatures: Vec<String>,
        compression: &str,
        packed_context_free_data: &str,
        packed_trx: &str
    ) -> Self {
        Self { 
            signatures,
            compression: compression.to_owned(),
            packed_context_free_data: packed_context_free_data.to_owned(),
            packed_trx: packed_trx.to_owned()
        }
    }
}

pub(crate) async fn get_account(url: &str, account_name: &str) -> Result<GetAccount, Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, GET_ACCOUNT_API);
    let param = GetAccountParam::new(account_name);

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let account_info: GetAccount = res.json().await?;
    
    Ok(account_info)
}

pub(crate) async fn cross_to_bifrost(url: &str, account_id: &str, amount: u128) -> Result<(), ()> {
    todo!();
}

pub(crate) async fn get_info(url: &str) -> Result<GetInfo, Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, GET_INFO_API);
    let param = GetInfoParams::default();

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let node_info: GetInfo = res.json().await?;
    
    Ok(node_info)
}

pub(crate) async fn get_block(url: &str, block_num_or_id: &str) -> Result<GetBlock, Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, GET_INFO_API);
    let param = GetBlockParams::new(block_num_or_id);

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let block_info: GetBlock = res.json().await?;
    
    Ok(block_info)
}

pub(crate) async fn push_transaction() -> Result<(), Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, GET_INFO_API);
    let param = GetBlockParams::new(block_num_or_id);

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let block_info: GetBlock = res.json().await?;
    
    Ok(block_info)
}