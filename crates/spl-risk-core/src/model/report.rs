use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub mint: Pubkey,
    pub risk_score: u32,
    pub confidence_score: f32, // 0.0-1.0 - how complete is the data
    pub profile: String,
    pub flags: RiskFlags,
    pub metrics: RiskMetrics,
    pub breakdown: Vec<RiskBreakdown>,
    pub summary: String,
    pub warnings: Vec<String>,
    pub data_sources: DataSources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSources {
    pub rpc: String,        // "ok", "timeout", "error"
    pub metadata: String,   // "ok", "cached", "missing", "error"
    pub holders: String,    // "ok", "partial", "cached", "timeout"
    pub wallet_age: String, // "ok", "cached", "missing"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_at: Option<HashMap<String, String>>, // timestamp for cached data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlags {
    pub mint_authority: bool,
    pub freeze_authority: bool,

    #[cfg(feature = "lp-analysis")]
    pub lp_detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub creator_supply_pct: f64,
    pub wallet_age_days: Option<u64>,
    pub holders: usize,
    pub decimals: Option<u8>,
    pub total_supply: Option<u64>,
    pub top_holder_pct: Option<f64>,

    #[cfg(feature = "lp-analysis")]
    pub total_lp_tvl: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBreakdown {
    pub rule: String,
    pub weight: i32,
    pub description: String,
    pub status: Option<String>,
}

impl RiskReport {
    pub fn new(mint: Pubkey, profile: String) -> Self {
        Self {
            mint,
            risk_score: 0,
            confidence_score: 1.0, // Start optimistic
            profile,
            flags: RiskFlags {
                mint_authority: false,
                freeze_authority: false,
                #[cfg(feature = "lp-analysis")]
                lp_detected: false,
            },
            metrics: RiskMetrics {
                creator_supply_pct: 0.0,
                wallet_age_days: None,
                holders: 0,
                decimals: None,     // ← добавили
                total_supply: None, // ← добавили
                top_holder_pct: None,

                #[cfg(feature = "lp-analysis")]
                total_lp_tvl: None,
            },
            breakdown: Vec::new(),
            summary: String::new(),
            warnings: Vec::new(),
            data_sources: DataSources {
                rpc: "ok".to_string(),
                metadata: "ok".to_string(),
                holders: "ok".to_string(),
                wallet_age: "ok".to_string(),
                cached_at: None,
            },
        }
    }

    pub fn add_rule(&mut self, rule: &str, weight: i32, description: &str, status: Option<&str>) {
        self.breakdown.push(RiskBreakdown {
            rule: rule.to_string(),
            weight,
            description: description.to_string(),
            status: status.map(|s| s.to_string()),
        });
    }

    pub fn calculate_score(&mut self) {
        let total: i32 = self.breakdown.iter().map(|b| b.weight).sum();
        self.risk_score = total.max(0).min(100) as u32;
    }

    pub fn update_confidence(&mut self) {
        let mut confidence = 0.95; // Базовый максимум 95%, не 100%

        // Reduce confidence based on data quality
        if self.data_sources.holders == "partial" || self.data_sources.holders == "timeout" {
            confidence *= 0.7; // -30% for partial holder data
        }

        if self.data_sources.metadata == "cached" {
            confidence *= 0.98; // -2% for cached metadata
        } else if self.data_sources.metadata == "missing" {
            confidence *= 0.85; // -15% for missing metadata
        }

        if self.data_sources.wallet_age == "missing" {
            confidence *= 0.92; // -8% for missing wallet age
        }

        if self.data_sources.rpc == "timeout" || self.data_sources.rpc == "error" {
            confidence *= 0.6; // -40% for RPC issues
        }

        // Community edition без LP analysis
        #[cfg(not(feature = "lp-analysis"))]
        {
            confidence *= 0.90; // -10% за отсутствие LP данных
        }

        self.confidence_score = confidence;
    }

    pub fn confidence_level(&self) -> &str {
        match (self.confidence_score * 100.0) as u32 {
            92..=100 => "HIGH",
            70..=91 => "MEDIUM",
            _ => "LOW",
        }
    }

    pub fn generate_summary(&mut self) {
        let base_summary = match self.risk_score {
            0..=20 => "Low risk. Token appears relatively safe based on on-chain data.".to_string(),
            21..=40 => "Low-medium risk. Some concerns present, proceed with caution.".to_string(),
            41..=60 => "Medium risk. Multiple risk factors detected. DYOR recommended.".to_string(),
            61..=80 => {
                "High risk. Significant red flags present. High probability of issues.".to_string()
            }
            _ => "Critical risk. Extreme caution advised. Strong rug-pull indicators.".to_string(),
        };

        self.summary = base_summary;

        if self.flags.mint_authority && self.flags.freeze_authority {
            self.summary
                .push_str(" Token owner retains destructive privileges.");
        }

        // Add warning if holder data is incomplete
        if self.metrics.holders == 0
            && (self.data_sources.holders == "partial" || self.data_sources.holders == "timeout")
        {
            if !self.warnings.contains(&"RPC returned zero holders - data is incomplete. Token may have many more holders.".to_string()) {
                self.warnings.push("RPC returned zero holders - data is incomplete. Token may have many more holders.".to_string());
            }
            self.summary
                .push_str(" Note: Unable to fetch complete holder data.");
        }

        // Add confidence warning if low
        if self.confidence_score < 0.7 {
            self.warnings.insert(0, format!(
                "Low confidence score ({:.0}%) - data may be incomplete. Results may be optimistic.",
                self.confidence_score * 100.0
            ));
        }
    }

    pub fn risk_level(&self) -> &str {
        match self.risk_score {
            0..=20 => "LOW",
            21..=40 => "LOW-MEDIUM",
            41..=60 => "MEDIUM",
            61..=80 => "HIGH",
            _ => "CRITICAL",
        }
    }
}
