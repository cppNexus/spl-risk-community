use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub weights: RiskWeights,
    pub thresholds: Thresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskWeights {
    // Critical (high weight)
    pub mint_authority_active: i32,
    pub freeze_authority_active: i32,
    pub creator_supply_high: i32,
    pub creator_is_authority: i32,

    // Medium
    pub wallet_young: i32,
    pub low_holders: i32,
    pub no_verified_metadata: i32,

    // Risk reducers
    pub mint_revoked: i32,
    pub freeze_revoked: i32,
    pub supply_distributed: i32,

    #[cfg(feature = "lp-analysis")]
    pub no_lp_detected: i32,
    #[cfg(feature = "lp-analysis")]
    pub low_lp_value: i32,
    #[cfg(feature = "lp-analysis")]
    pub lp_not_locked: i32,
    #[cfg(feature = "lp-analysis")]
    pub lp_burned: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thresholds {
    pub creator_supply_high_pct: f64,
    pub supply_distributed_pct: f64,
    pub wallet_young_days: u64,
    pub low_holders_count: u64,

    #[cfg(feature = "lp-analysis")]
    pub low_lp_value_usd: f64,
}

impl Config {
    pub fn from_profile(profile: &str) -> Result<Self> {
        match profile {
            "conservative" => Ok(Self::conservative()),
            "balanced" => Ok(Self::balanced()),
            "degenerate" => Ok(Self::degenerate()),
            _ => anyhow::bail!("Unknown profile: {}", profile),
        }
    }

    pub fn conservative() -> Self {
        Self {
            weights: RiskWeights {
                mint_authority_active: 35,
                freeze_authority_active: 30,
                creator_supply_high: 30,
                creator_is_authority: 20,
                wallet_young: 15,
                low_holders: 10,
                no_verified_metadata: 5,
                mint_revoked: -25,
                freeze_revoked: -20,
                supply_distributed: -20,

                #[cfg(feature = "lp-analysis")]
                no_lp_detected: 25,
                #[cfg(feature = "lp-analysis")]
                low_lp_value: 20,
                #[cfg(feature = "lp-analysis")]
                lp_not_locked: 30,
                #[cfg(feature = "lp-analysis")]
                lp_burned: -25,
            },
            thresholds: Thresholds {
                creator_supply_high_pct: 40.0,
                supply_distributed_pct: 15.0,
                wallet_young_days: 14,
                low_holders_count: 50,

                #[cfg(feature = "lp-analysis")]
                low_lp_value_usd: 5000.0,
            },
        }
    }

    pub fn balanced() -> Self {
        Self {
            weights: RiskWeights {
                mint_authority_active: 30,
                freeze_authority_active: 25,
                creator_supply_high: 25,
                creator_is_authority: 15,
                wallet_young: 10,
                low_holders: 5,
                no_verified_metadata: 2,
                mint_revoked: -20,
                freeze_revoked: -15,
                supply_distributed: -15,

                #[cfg(feature = "lp-analysis")]
                no_lp_detected: 20,
                #[cfg(feature = "lp-analysis")]
                low_lp_value: 15,
                #[cfg(feature = "lp-analysis")]
                lp_not_locked: 25,
                #[cfg(feature = "lp-analysis")]
                lp_burned: -20,
            },
            thresholds: Thresholds {
                creator_supply_high_pct: 50.0,
                supply_distributed_pct: 10.0,
                wallet_young_days: 7,
                low_holders_count: 30,

                #[cfg(feature = "lp-analysis")]
                low_lp_value_usd: 2000.0,
            },
        }
    }

    pub fn degenerate() -> Self {
        Self {
            weights: RiskWeights {
                mint_authority_active: 20,
                freeze_authority_active: 15,
                creator_supply_high: 15,
                creator_is_authority: 10,
                wallet_young: 5,
                low_holders: 3,
                no_verified_metadata: 1,
                mint_revoked: -15,
                freeze_revoked: -10,
                supply_distributed: -10,

                #[cfg(feature = "lp-analysis")]
                no_lp_detected: 10,
                #[cfg(feature = "lp-analysis")]
                low_lp_value: 8,
                #[cfg(feature = "lp-analysis")]
                lp_not_locked: 15,
                #[cfg(feature = "lp-analysis")]
                lp_burned: -15,
            },
            thresholds: Thresholds {
                creator_supply_high_pct: 70.0,
                supply_distributed_pct: 5.0,
                wallet_young_days: 3,
                low_holders_count: 10,

                #[cfg(feature = "lp-analysis")]
                low_lp_value_usd: 500.0,
            },
        }
    }
}
