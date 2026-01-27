mod authorities;
mod supply;
mod metadata;

// #[cfg(test)]
// mod tests;

pub use authorities::*;
pub use supply::*;
pub use metadata::*;

pub fn get_community_rules() -> Vec<Box<dyn spl_risk_core::heuristics::RiskRule>> {
    let mut rules: Vec<Box<dyn spl_risk_core::heuristics::RiskRule>> = Vec::new();

    rules.push(Box::new(MintAuthorityRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(FreezeAuthorityRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(CreatorSupplyRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(CreatorIsAuthorityRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(WalletAgeRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(HolderCountRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    rules.push(Box::new(VerifiedMetadataRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);

    rules
}

#[cfg(feature = "pro")]
pub fn get_pro_rules() -> Vec<Box<dyn spl_risk_core::heuristics::RiskRule>> {
    let mut rules: Vec<Box<dyn spl_risk_core::heuristics::RiskRule>> = get_community_rules();

    #[cfg(feature = "lp-analysis")]
    {
        rules.push(Box::new(crate::heuristics::lp::LpDetectionRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
        rules.push(Box::new(crate::heuristics::lp::LpValueRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
        rules.push(Box::new(crate::heuristics::lp::LpLockRule) as Box<dyn spl_risk_core::heuristics::RiskRule>);
    }

    rules
}
