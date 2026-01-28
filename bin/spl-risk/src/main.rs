use anyhow::Result;
use clap::Parser;

mod cli;

use cli::Cli;
use spl_risk_core::config::Config;
use spl_risk_core::scoring::RiskAnalyzer;
use spl_risk_rpc::SolanaRpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = Config::from_profile(&cli.profile)?;

    // Initialize RPC client
    let rpc_client = SolanaRpcClient::new(&cli.rpc_url, cli.timeout)?;

    // Clear cache if requested
    if cli.no_cache {
        rpc_client.clear_cache();
    }

    // Show cache stats if requested
    if cli.cache_stats {
        let stats = rpc_client.cache_stats();
        println!("Cache Statistics:");
        println!(
            "  Token Cache: {}/{} entries ({} expired)",
            stats.token_cache.size, stats.token_cache.capacity, stats.token_cache.expired_entries
        );
        println!(
            "  Metadata Cache: {}/{} entries ({} expired)",
            stats.metadata_cache.size,
            stats.metadata_cache.capacity,
            stats.metadata_cache.expired_entries
        );
        println!(
            "  Wallet Age Cache: {}/{} entries ({} expired)",
            stats.wallet_age_cache.size,
            stats.wallet_age_cache.capacity,
            stats.wallet_age_cache.expired_entries
        );
        println!();
    }

    // Create analyzer
    let rules = spl_risk_community::community_rules();
    let analyzer = RiskAnalyzer::new(config, rpc_client, rules);

    // Analyze token
    let report = analyzer.analyze(&cli.mint_address).await?;

    // Output results
    if cli.json {
        spl_risk_output::json::print_report(&report)?;
    } else {
        spl_risk_output::human::print_report(&report, cli.verbose)?;
    }

    // Exit with appropriate code
    let exit_code = if report.risk_score >= 70 {
        2 // High risk
    } else if report.risk_score >= 40 {
        1 // Medium risk
    } else {
        0 // Low risk
    };

    std::process::exit(exit_code);
}
