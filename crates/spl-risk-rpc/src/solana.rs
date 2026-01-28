use crate::cache::Cache;
use anyhow::Result;
use async_trait::async_trait;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_program::program_option::COption;
use solana_sdk::pubkey::Pubkey;
use spl_risk_core::error::RiskError;
use spl_risk_core::model::token::TokenData;
use spl_risk_core::model::token::TokenHolder;
use spl_risk_core::model::token::TokenMetadata;
use spl_risk_core::provider::TokenDataProvider;
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::state::{Account as TokenAccount, Mint};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

pub struct SolanaRpcClient {
    client: Arc<RpcClient>,
    // Caches with TTL
    token_cache: Cache<TokenData>,
    metadata_cache: Cache<TokenMetadata>,
    wallet_age_cache: Cache<u64>,
}

impl SolanaRpcClient {
    pub fn new(url: &str, _timeout: Duration) -> Result<Self> {
        let client = Arc::new(RpcClient::new_with_commitment(
            url.to_string(),
            CommitmentConfig::confirmed(),
        ));

        Ok(Self {
            client,
            token_cache: Cache::new(Duration::from_secs(300), 1000),
            metadata_cache: Cache::new(Duration::from_secs(300), 1000),
            wallet_age_cache: Cache::new(Duration::from_secs(600), 5000),
        })
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.token_cache.clear();
        self.metadata_cache.clear();
        self.wallet_age_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStatsSummary {
        CacheStatsSummary {
            token_cache: self.token_cache.stats(),
            metadata_cache: self.metadata_cache.stats(),
            wallet_age_cache: self.wallet_age_cache.stats(),
        }
    }

    pub async fn fetch_token_data(&self, mint: &Pubkey) -> Result<TokenData> {
        // Check cache first
        if let Some(cached) = self.token_cache.get(mint) {
            return Ok(cached);
        }

        // Fetch mint account
        let mint_account = self
            .client
            .get_account(mint)
            .await
            .map_err(|e| RiskError::RpcError(e.to_string()))?;

        // Verify it's an SPL token (check both Token and Token-2022 programs)
        let token_program_id = spl_token_2022::id();
        let owner_pubkey = Pubkey::from(mint_account.owner.to_bytes());
        let program_pubkey = Pubkey::from(token_program_id.to_bytes());

        // Also check legacy spl-token program ID
        let legacy_token_program = Pubkey::from([
            6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180,
            133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
        ]);

        if owner_pubkey != program_pubkey && owner_pubkey != legacy_token_program {
            return Err(RiskError::NotSplToken.into());
        }

        // Parse mint data using StateWithExtensions
        let mint_data = StateWithExtensions::<Mint>::unpack(&mint_account.data)
            .map_err(|e| RiskError::ParseError(e.to_string()))?;

        // Fetch data in parallel using tokio::join!
        let (holders_result, metadata_result, creation_time_result) = tokio::join!(
            self.fetch_token_holders(mint, mint_data.base.supply, mint_data.base.decimals),
            self.fetch_metadata(mint),
            self.fetch_creation_time(mint),
        );

        let holders = holders_result?;
        let metadata = metadata_result.ok();
        let creation_timestamp = creation_time_result.ok();

        // Convert COption to Option<Pubkey>
        let mint_authority = match mint_data.base.mint_authority {
            COption::Some(key) => Some(Pubkey::from(key.to_bytes())),
            COption::None => None,
        };

        let freeze_authority = match mint_data.base.freeze_authority {
            COption::Some(key) => Some(Pubkey::from(key.to_bytes())),
            COption::None => None,
        };

        let token_data = TokenData {
            mint: *mint,
            supply: mint_data.base.supply,
            decimals: mint_data.base.decimals,
            mint_authority,
            freeze_authority,
            metadata,
            holders,
            creation_timestamp,

            #[cfg(feature = "lp-analysis")]
            lp_pools: Vec::new(),
        };

        // Cache the result
        self.token_cache.insert(*mint, token_data.clone());

        Ok(token_data)
    }

    async fn fetch_token_holders(
        &self,
        mint: &Pubkey,
        total_supply: u64,
        _decimals: u8,
    ) -> Result<Vec<TokenHolder>> {
        eprintln!("Fetching top token holders...");

        // Retry с экспоненциальным backoff
        let mut retries = 3;
        let mut delay = Duration::from_secs(2);

        let largest = loop {
            match self.client.get_token_largest_accounts(mint).await {
                Ok(accounts) => break accounts,
                Err(e) => {
                    if retries == 0 {
                        eprintln!("Failed to fetch largest accounts after retries: {}", e);
                        return Err(RiskError::RpcError(e.to_string()).into());
                    }

                    eprintln!(
                        "Rate limited, retrying in {:?}... ({} retries left)",
                        delay, retries
                    );
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff: 2s, 4s, 8s
                    retries -= 1;
                }
            }
        };

        if largest.is_empty() {
            eprintln!("Warning: No holders found for this token");
            return Ok(Vec::new());
        }

        eprintln!("Found {} top holders", largest.len());

        let mut holders = Vec::new();

        for account_info in largest {
            let amount = account_info
                .amount
                .amount
                .parse::<u64>()
                .unwrap_or_else(|e| {
                    eprintln!("Failed to parse amount: {}", e);
                    0
                });

            if amount > 0 {
                let percentage = (amount as f64 / total_supply as f64) * 100.0;

                let token_account_addr = match Pubkey::from_str(&account_info.address) {
                    Ok(addr) => addr,
                    Err(_) => continue,
                };

                // Добавить небольшую задержку между запросами владельцев
                tokio::time::sleep(Duration::from_millis(100)).await;

                let owner_pubkey = match self.get_token_account_owner(&token_account_addr).await {
                    Ok(owner) => owner,
                    Err(e) => {
                        eprintln!("Failed to get owner for {}: {}", token_account_addr, e);
                        token_account_addr
                    }
                };

                holders.push(TokenHolder {
                    address: owner_pubkey,
                    amount,
                    percentage,
                    wallet_age_days: None,
                });
            }
        }

        holders.sort_by(|a, b| b.amount.cmp(&a.amount));

        Ok(holders)
    }

    async fn get_token_account_owner(&self, token_account: &Pubkey) -> Result<Pubkey> {
        let account = self
            .client
            .get_account(token_account)
            .await
            .map_err(|e| RiskError::RpcError(e.to_string()))?;

        let state = StateWithExtensions::<TokenAccount>::unpack(&account.data)
            .map_err(|e| RiskError::ParseError(e.to_string()))?;

        Ok(Pubkey::from(state.base.owner.to_bytes()))
    }

    async fn fetch_metadata(&self, mint: &Pubkey) -> Result<TokenMetadata> {
        // Check cache first
        if let Some(cached) = self.metadata_cache.get(mint) {
            return Ok(cached);
        }

        // Derive metadata PDA using Metaplex standard
        let metadata_seeds = &[b"metadata", mpl_token_metadata::ID.as_ref(), mint.as_ref()];

        let (metadata_pda, _) =
            Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);

        // Fetch metadata account
        let account = self
            .client
            .get_account(&metadata_pda)
            .await
            .map_err(|e| RiskError::RpcError(e.to_string()))?;

        // Verify owner is Metaplex program
        if account.owner != mpl_token_metadata::ID {
            return Err(RiskError::ParseError(
                "Account is not owned by Metaplex program".to_string(),
            )
            .into());
        }

        // Try to parse metadata - handle different mpl_token_metadata versions
        // For mpl-token-metadata 5.x, the structure might be different
        // We'll extract basic info manually if needed

        // Simple parsing for name, symbol, uri (first ~100 bytes typically contain this)
        let data = &account.data;

        // Metadata structure (simplified):
        // - key (1 byte)
        // - update_authority (32 bytes)
        // - mint (32 bytes)
        // - name (4 + 32 bytes string)
        // - symbol (4 + 10 bytes string)
        // - uri (4 + 200 bytes string)

        let mut offset = 1 + 32 + 32; // Skip key, update_authority, mint

        let name = Self::read_string(data, &mut offset)?;
        let symbol = Self::read_string(data, &mut offset)?;
        let uri = Self::read_string(data, &mut offset)?;

        // For verified check, we'd need to parse collection info
        // For now, default to false (can be enhanced later)
        let is_verified = false;

        let token_metadata = TokenMetadata {
            name: name.trim_matches('\0').to_string(),
            symbol: symbol.trim_matches('\0').to_string(),
            uri: uri.trim_matches('\0').to_string(),
            is_verified,
        };

        // Cache the result
        self.metadata_cache.insert(*mint, token_metadata.clone());

        Ok(token_metadata)
    }

    fn read_string(data: &[u8], offset: &mut usize) -> Result<String> {
        if *offset + 4 > data.len() {
            return Ok(String::new());
        }

        let len = u32::from_le_bytes([
            data[*offset],
            data[*offset + 1],
            data[*offset + 2],
            data[*offset + 3],
        ]) as usize;

        *offset += 4;

        if *offset + len > data.len() {
            return Ok(String::new());
        }

        let string = String::from_utf8_lossy(&data[*offset..*offset + len]).to_string();
        *offset += len;

        Ok(string)
    }

    async fn fetch_creation_time(&self, mint: &Pubkey) -> Result<i64> {
        use solana_client::rpc_config::RpcTransactionConfig;
        use solana_transaction_status::UiTransactionEncoding;

        let _config = RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };

        let signatures = self
            .client
            .get_signatures_for_address(mint)
            .await
            .map_err(|e| RiskError::RpcError(e.to_string()))?;

        if let Some(sig) = signatures.last() {
            if let Some(block_time) = sig.block_time {
                return Ok(block_time);
            }
        }

        Err(RiskError::ParseError("No creation time found".to_string()).into())
    }

    async fn get_wallet_age(&self, wallet: &Pubkey) -> Result<u64> {
        // Check cache first
        if let Some(cached) = self.wallet_age_cache.get(wallet) {
            return Ok(cached);
        }

        let signatures = self
            .client
            .get_signatures_for_address(wallet)
            .await
            .map_err(|e| RiskError::RpcError(e.to_string()))?;

        if let Some(first_tx) = signatures.last() {
            if let Some(block_time) = first_tx.block_time {
                let now = chrono::Utc::now().timestamp();
                let age_seconds = now - block_time;
                let age_days = (age_seconds / 86400).max(0) as u64;

                // Cache the result
                self.wallet_age_cache.insert(*wallet, age_days);

                return Ok(age_days);
            }
        }

        Ok(0)
    }

    /// Populate wallet ages for top holders
    pub async fn enrich_holder_ages(&self, holders: &mut [TokenHolder]) -> Result<()> {
        // Only check top 10 holders to avoid too many RPC calls
        let limit = holders.len().min(10);

        for holder in holders.iter_mut().take(limit) {
            if holder.wallet_age_days.is_none() {
                if let Ok(age) = self.get_wallet_age(&holder.address).await {
                    holder.wallet_age_days = Some(age);
                }
            }
        }

        Ok(())
    }
}

// Placeholder for mpl_token_metadata
mod mpl_token_metadata {
    use solana_sdk::pubkey::Pubkey;

    pub const ID: Pubkey = solana_sdk::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
}

use crate::cache::CacheStats;

#[derive(Debug, Clone)]
pub struct CacheStatsSummary {
    pub token_cache: CacheStats,
    pub metadata_cache: CacheStats,
    pub wallet_age_cache: CacheStats,
}

impl Clone for SolanaRpcClient {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            token_cache: self.token_cache.clone(),
            metadata_cache: self.metadata_cache.clone(),
            wallet_age_cache: self.wallet_age_cache.clone(),
        }
    }
}

#[async_trait]
impl TokenDataProvider for SolanaRpcClient {
    async fn fetch_token_data(&self, mint: &Pubkey) -> Result<TokenData> {
        SolanaRpcClient::fetch_token_data(self, mint).await
    }

    async fn enrich_holder_ages(&self, holders: &mut [TokenHolder]) -> Result<()> {
        SolanaRpcClient::enrich_holder_ages(self, holders).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_pda_derivation() {
        let mint = Pubkey::new_unique();
        let metadata_seeds = &[b"metadata", mpl_token_metadata::ID.as_ref(), mint.as_ref()];

        let (pda, _bump) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);

        // PDA should be valid and non-default
        assert_ne!(pda, Pubkey::default());
    }

    #[tokio::test]
    async fn test_rpc_client_creation() {
        let result = SolanaRpcClient::new(
            "https://api.mainnet-beta.solana.com",
            Duration::from_secs(10),
        );

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_clone() {
        let client = SolanaRpcClient::new(
            "https://api.mainnet-beta.solana.com",
            Duration::from_secs(10),
        )
        .unwrap();

        let cloned = client.clone();

        // Both should share the same Arc<RpcClient>
        assert!(Arc::ptr_eq(&client.client, &cloned.client));
    }
}
