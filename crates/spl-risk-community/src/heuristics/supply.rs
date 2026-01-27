use spl_risk_core::config::Config;
use spl_risk_core::heuristics::RiskRule;
use spl_risk_core::model::{RiskReport, TokenData};

pub struct CreatorSupplyRule;

impl RiskRule for CreatorSupplyRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        let creator_pct = token.creator_supply_percentage();
        report.metrics.creator_supply_pct = creator_pct;
        
        if creator_pct > config.thresholds.creator_supply_high_pct {
            report.add_rule(
                "creator_supply_high",
                config.weights.creator_supply_high,
                &format!("Creator holds {:.1}% of supply (high concentration)", creator_pct),
                Some("high"),  // ← лучше "high"
            );
        } else if creator_pct < config.thresholds.supply_distributed_pct {
            report.add_rule(
                "supply_distributed",
                config.weights.supply_distributed,
                &format!("Top holder has only {:.1}% (well distributed)", creator_pct),
                Some("low"),   // ← лучше "low" или "distributed"
            );
        }
        // можно добавить нейтральный случай, если нужно
    }
    
    fn name(&self) -> &str {
        "creator_supply"
    }
}

pub struct HolderCountRule;

impl RiskRule for HolderCountRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        let holder_count = token.holder_count();
        report.metrics.holders = holder_count;
        
        if (holder_count as u64) < config.thresholds.low_holders_count {
            let description = if holder_count <= 10 {
                format!("Very low holder count ({}) - high concentration risk", holder_count)
            } else if holder_count <= 50 {
                format!("Low holder count ({}) - early stage or limited adoption", holder_count)
            } else {
                format!("Moderate holder count ({}) - developing adoption", holder_count)
            };
            
            report.add_rule(
                "low_holders",
                config.weights.low_holders,
                &description,
                Some("low"),   // ← правильно "low"
            );
        }
    }
    
    fn name(&self) -> &str {
        "holder_count"
    }
}

pub struct WalletAgeRule;

impl RiskRule for WalletAgeRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        if let Some(creator) = token.holders.first() {
            if let Some(age_days) = creator.wallet_age_days {
                report.metrics.wallet_age_days = Some(age_days);
                
                if age_days < config.thresholds.wallet_young_days {
                    report.add_rule(
                        "wallet_young",
                        config.weights.wallet_young,
                        &format!("Creator wallet is only {} days old", age_days),
                        Some("young"),   // ← лучше "young" или "new"
                    );
                }
            }
        }
    }
    
    fn name(&self) -> &str {
        "wallet_age"
    }
}