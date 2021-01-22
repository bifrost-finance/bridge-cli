use core::marker::PhantomData;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::traits::{AtLeast32Bit, Member, MaybeSerializeDeserialize};
use sp_runtime::AccountId32;
use subxt::{
	PairSigner, DefaultNodeRuntime as BifrostRuntime, Call, Client,
	system::{AccountStoreExt, System, SystemEventsDecoder}, Encoded, Event, Store,
};

pub mod bridge_eos {
    use super::*;

    #[subxt::module]
    pub trait BridgeEos: System {
        type Balance: Member
            + AtLeast32Bit
            + codec::Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + std::fmt::Debug
            + From<<Self as System>::BlockNumber>;
    }

    impl BridgeEos for BifrostRuntime {
        type Balance = u128;
    }

    #[derive(Clone, Debug, PartialEq, Call, Encode)]
    pub struct CrossToEosCall<T: BridgeEos> {
        pub to: Vec<u8>,
        #[codec(compact)]
        pub amount: T::Balance,
        pub memo: Vec<u8>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
    pub struct SentCrossChainTransactionEvent<T: BridgeEos> {
        pub place_holder: PhantomData<T>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
    pub struct FailToSendCrossChainTransactionEvent<T: BridgeEos> {
        pub place_holder: PhantomData<T>,
    }

    #[allow(dead_code)]
    pub async fn cross_to_eos(
        url: &str,
        from: &PairSigner::<BifrostRuntime, Pair>,
        to: Vec<u8>,
        amount: u128,
        memo: Vec<u8>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
            .set_url(url)
            .build()
            .await?;

        let call = CrossToEosCall {
            to,
            amount,
            memo,
        };
        let extrinsic = client.create_signed(call, from).await?;
        
        let mut decoder = client.events_decoder::<CrossToEosCall<BifrostRuntime>>();
        decoder.with_bridge_eos();
        
        let bridge_eos_events = client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
        let event = bridge_eos_events
            .find_event::<SentCrossChainTransactionEvent::<BifrostRuntime>>()?
            .ok_or("No Event found or decoded.")?;
        let block_hash = bridge_eos_events.block;

        Ok(block_hash.to_string())
    }
}

pub mod assets {
    use super::*;

    #[subxt::module]
    pub trait Assets: System {
        type Balance: Member
            + AtLeast32Bit
            + codec::Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + std::fmt::Debug
            + From<<Self as System>::BlockNumber>;
        type AssetId: Member 
            + Default
            + codec::Codec
            + AtLeast32Bit
            + Copy
            + MaybeSerializeDeserialize;
    }

    impl Assets for BifrostRuntime {
        type Balance = u128;
        type AssetId = u32;
    }

    #[derive(Clone, Debug, Eq, PartialEq, Store, Encode, Decode)]
    pub struct AccountAssetsStore<'a, T: Assets> {
        #[store(returns = AccountAsset<T>)]
        /// according account to get asset
        pub account_id: (T::AssetId, &'a T::AccountId),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Decode, Encode)]
    pub struct AccountAsset<T: Assets> {
        pub balance: T::Balance,
        pub locked: T::Balance,
        pub available: T::Balance,
    }

    pub async fn get_asset_by_account(
        url: &str,
        who: &AccountId32,
        asset_id: u32
    ) -> Result<AccountAsset<BifrostRuntime>, Box<dyn std::error::Error>> {
        let client: Client<BifrostRuntime> = subxt::ClientBuilder::new()
            .set_url(url)
            .build()
            .await?;

        let asset = client.account_assets((asset_id, &who.clone().into()), None).await?;

        // let mut iter = client.account_assets_iter(None).await?;
        // let mut index = 0u32;
        // while let Some((key, val)) = iter.next().await? {
        //     let k: Result<(u32, AccountId32), _> = serde_json::from_slice(&key.0);
        //     println!("key: {:?}, value: {:?}", key, val);
        //     index += 1;
        // }
        // println!("length: {:?}", index);

        Ok(asset)
    }
}