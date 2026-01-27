use spl_risk_core::config::Config;
use spl_risk_core::heuristics::RiskRule;
use spl_risk_core::model::{RiskReport, TokenData};

pub struct MintAuthorityRule;

impl RiskRule for MintAuthorityRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        if token.mint_authority.is_some() {
            report.flags.mint_authority = true;
            report.add_rule(
                "mint_authority_active",
                config.weights.mint_authority_active,
                "Mint authority is active - owner can create unlimited tokens",
                Some("active"),
            );
        } else {
            report.add_rule(
                "mint_revoked",
                config.weights.mint_revoked,
                "Mint authority revoked - supply is fixed",
                Some("revoked"),
            );
        }
    }
    
    fn name(&self) -> &str {
        "mint_authority"
    }
}

pub struct FreezeAuthorityRule;

impl RiskRule for FreezeAuthorityRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        if token.freeze_authority.is_some() {
            report.flags.freeze_authority = true;
            report.add_rule(
                "freeze_authority_active",
                config.weights.freeze_authority_active,
                "Freeze authority is active - owner can freeze token accounts",
                Some("active")
            );
        } else {
            report.add_rule(
                "freeze_revoked",
                config.weights.freeze_revoked,
                "Freeze authority revoked - accounts cannot be frozen",
                Some("revoked")
            );
        }
    }
    
    fn name(&self) -> &str {
        "freeze_authority"
    }
}

pub struct CreatorIsAuthorityRule;

impl RiskRule for CreatorIsAuthorityRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        if let Some(creator) = token.creator_address() {
            let is_mint_authority = token.mint_authority
                .map(|auth| auth == creator)
                .unwrap_or(false);
            
            let is_freeze_authority = token.freeze_authority
                .map(|auth| auth == creator)
                .unwrap_or(false);
            
            if is_mint_authority || is_freeze_authority {
                report.add_rule(
                    "creator_is_authority",
                    config.weights.creator_is_authority,
                    "Token creator retains mint or freeze authority",
                    Some("retains")
                );
            }
        }
    }
    
    fn name(&self) -> &str {
        "creator_is_authority"
    }
}
