use eos_chain::{
    Action, Asset, Checksum256, Read, SerializeData,
    Signature, Transaction, TimePointSec, SignedTransaction
};
use eos_keys::secret::SecretKey;
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
    pub net_weight: u32,
    pub cpu_weight: u32,
    pub ram_usage: u32,
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
        signed_trx: SignedTransaction
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            signatures: signed_trx.signatures.iter().map(|sig| sig.to_string()).collect(),
            compression: "none".to_string(),
            packed_context_free_data: "".to_string(),
            packed_trx: hex::encode(
                &signed_trx.trx.to_serialize_data().map_err(|_| "Failed serialzie transaction data.")?
            )
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushTransaction {
    pub transaction_id: String,
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

pub(crate) async fn cross_to_bifrost(
    url: &str,
    from: &str,
    to: &str,
    amount: &str,
    memo: &str,
    pk: String
) -> Result<(), Box<dyn error::Error>> {
    // let pks = pk.iter().map(|s| {
    //     SecretKey::from_wif(s).unwrap()
    // }).collect::<SecretKey>();
    // let sk = SecretKey::from_wif(&pk).map_err(|_| "Failed to parse private key.".to_owned())?;
    let sk = pk;

    // get node info
    let node_info: GetInfo = self::get_info(url).await?;

    let chain_id = node_info.chain_id;
    let block_num = node_info.head_block_id;

    let block_info: GetBlock = self::get_block(url, &block_num).await?;

    let ref_block_num = (block_info.block_num & 0xffff) as u16;
    let ref_block_prefix = block_info.ref_block_prefix as u32;

    let action = Action::transfer(from, to, amount, memo,)
        .map_err(|_| "Failed to create action for transaction.".to_owned())?;
    let actions = vec![action];

    let expiration = TimePointSec::now().sec_since_epoch() + 300;
    let trx = Transaction::new(expiration, ref_block_num, ref_block_prefix, actions);
    let signature = trx.generate_signature(sk, &chain_id)
        .map_err(|_| "Failed to sign for this transaction.".to_owned())?;
    let signatures = vec![signature];
    let	signed_trx = trx.generate_signed_transaction(signatures);

    let response = self::push_transaction(url, signed_trx).await?;

    Ok(())
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

pub(crate) async fn get_block(
    url: &str,
    block_num_or_id: &str
) -> Result<GetBlock, Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, GET_BLOCK_API);
    let param = GetBlockParams::new(block_num_or_id);

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let block_info: GetBlock = res.json().await?;
    
    Ok(block_info)
}

pub(crate) async fn push_transaction(
    url: &str,
    signed_trx: SignedTransaction
) -> Result<PushTransaction, Box<dyn error::Error>> {
    let full_url = format!("{}{}", url, PUSH_TRANSACTION_API);
    let param = PushTransactionParams::new(signed_trx)?;

    let client = reqwest::Client::new();
    let res = client.post(&full_url)
        .json(&param)
        .send()
        .await?;

    let transaction_info: PushTransaction = res.json().await?;
    
    Ok(transaction_info)
}
