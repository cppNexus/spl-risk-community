use spl_risk_core::config::Config;
use spl_risk_core::heuristics::RiskRule;
use spl_risk_core::model::{RiskReport, TokenData};

pub struct VerifiedMetadataRule;

impl RiskRule for VerifiedMetadataRule {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport) {
        match &token.metadata {
            Some(metadata) => {
                if metadata.is_verified {
                    report.add_rule(
                        "verified_metadata",
                        0, // нейтральный вес или отрицательный бонус
                        "Metadata is verified",
                        Some("verified"), // ← зелёный флаг
                    );
                } else {
                    report.add_rule(
                        "no_verified_metadata",
                        config.weights.no_verified_metadata,
                        "Metadata exists but is not verified",
                        Some("unverified"),
                    );
                }
            }
            None => {
                report.add_rule(
                    "no_metadata",
                    config.weights.no_verified_metadata, // или отдельный вес
                    "No metadata found for this token",
                    Some("missing"),
                );
            }
        }
    }

    fn name(&self) -> &str {
        "verified_metadata"
    }
}
