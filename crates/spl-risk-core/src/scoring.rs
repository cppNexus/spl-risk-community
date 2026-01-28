use crate::config::Config;
use crate::heuristics::RiskRule;
use crate::model::RiskReport;
use crate::provider::TokenDataProvider;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

pub struct RiskAnalyzer<P: TokenDataProvider> {
    config: Config,
    provider: P,
    rules: Vec<Box<dyn RiskRule>>,
}

impl<P: TokenDataProvider> RiskAnalyzer<P> {
    pub fn new(config: Config, provider: P, rules: Vec<Box<dyn RiskRule>>) -> Self {
        Self {
            config,
            provider,
            rules,
        }
    }

    pub async fn analyze(&self, mint: &Pubkey) -> Result<RiskReport> {
        // Fetch token data (cached if available)
        let mut token_data = self.provider.fetch_token_data(mint).await?;

        // Create report
        let mut report = RiskReport::new(*mint, self.config_profile());

        report.metrics.total_supply = Some(token_data.supply);
        report.metrics.decimals = Some(token_data.decimals);

        // Top holder % — самый большой процент (holders должны быть отсортированы по убыванию)
        if let Some(top_holder) = token_data.holders.first() {
            report.metrics.top_holder_pct = Some(top_holder.percentage);
        } else {
            // опционально: если holders пустой — можно залогировать или оставить None
            eprintln!("DEBUG: No holders found for mint {}", mint);
        }

        // Track data source status
        if token_data.holders.is_empty() {
            report.data_sources.holders = "partial".to_string();
        }

        if token_data.metadata.is_none() {
            report.data_sources.metadata = "missing".to_string();
        }

        // Enrich top holder wallet ages (only top 10 to limit RPC calls)
        if let Err(_) = self
            .provider
            .enrich_holder_ages(&mut token_data.holders)
            .await
        {
            report.data_sources.wallet_age = "missing".to_string();
        }

        // Apply rules
        for rule in &self.rules {
            rule.evaluate(&token_data, &self.config, &mut report);
        }

        // Calculate final score
        report.calculate_score();

        // Update confidence based on data quality
        report.update_confidence();

        // Generate summary
        report.generate_summary();

        Ok(report)
    }

    fn config_profile(&self) -> String {
        // Determine profile based on thresholds
        if self.config.thresholds.creator_supply_high_pct <= 40.0 {
            "conservative".to_string()
        } else if self.config.thresholds.creator_supply_high_pct >= 70.0 {
            "degenerate".to_string()
        } else {
            "balanced".to_string()
        }
    }
}
