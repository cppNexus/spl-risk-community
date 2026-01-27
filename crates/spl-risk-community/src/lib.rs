pub mod heuristics;

pub fn community_rules() -> Vec<Box<dyn spl_risk_core::heuristics::RiskRule>> {
    heuristics::get_community_rules()
}