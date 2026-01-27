use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use crate::model::TokenData;
use crate::model::token::TokenHolder;

#[async_trait]
pub trait TokenDataProvider: Send + Sync {
    async fn fetch_token_data(&self, mint: &Pubkey) -> Result<TokenData>;

    async fn enrich_holder_ages(&self, holders: &mut [TokenHolder]) -> Result<()>;
}
