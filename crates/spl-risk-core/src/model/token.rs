use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub mint: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub mint_authority: Option<Pubkey>,
    pub freeze_authority: Option<Pubkey>,
    pub metadata: Option<TokenMetadata>,
    pub holders: Vec<TokenHolder>,
    pub creation_timestamp: Option<i64>,

    #[cfg(feature = "lp-analysis")]
    pub lp_pools: Vec<LiquidityPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenHolder {
    pub address: Pubkey,
    pub amount: u64,
    pub percentage: f64,
    pub wallet_age_days: Option<u64>,
}

#[cfg(feature = "lp-analysis")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub dex: String,
    pub pool_address: Pubkey,
    pub tvl_usd: f64,
    pub lp_locked: bool,
    pub lp_burned: bool,
    pub creator_is_lp_provider: bool,
}

impl TokenData {
    pub fn total_supply(&self) -> u64 {
        self.supply
    }

    pub fn holder_count(&self) -> usize {
        self.holders.len()
    }

    pub fn creator_address(&self) -> Option<Pubkey> {
        self.holders.first().map(|h| h.address)
    }

    pub fn creator_supply_percentage(&self) -> f64 {
        self.holders.first().map(|h| h.percentage).unwrap_or(0.0)
    }

    pub fn is_supply_concentrated(&self, threshold: f64) -> bool {
        self.creator_supply_percentage() > threshold
    }
}
