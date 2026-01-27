use clap::Parser;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    name = "spl-risk",
    version,
    about = "Deterministic risk analyzer for Solana SPL tokens",
    long_about = "Analyzes SPL tokens on Solana and calculates risk score based on on-chain heuristics.\n\n\
                  ⚠️  DISCLAIMER: This tool provides probabilistic risk assessment. NOT financial advice.\n\
                  Users must conduct independent research (DYOR)."
)]
pub struct Cli {
    /// SPL token mint address to analyze
    #[arg(value_parser = parse_pubkey)]
    pub mint_address: Pubkey,
    
    /// Solana RPC endpoint URL
    #[arg(
        short = 'r',
        long,
        env = "SOLANA_RPC_URL",
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    pub rpc_url: String,
    
    /// Risk profile: conservative, balanced, or degenerate
    #[arg(
        short = 'p',
        long,
        default_value = "balanced",
        value_parser = ["conservative", "balanced", "degenerate"]
    )]
    pub profile: String,
    
    /// Output results as JSON
    #[arg(short = 'j', long)]
    pub json: bool,
    
    /// Show detailed breakdown
    #[arg(short = 'v', long)]
    pub verbose: bool,
    
    /// Request timeout in seconds
    #[arg(short = 't', long, default_value = "10", value_parser = parse_duration)]
    pub timeout: Duration,
    
    /// Disable caching
    #[arg(long)]
    pub no_cache: bool,
    
    /// Show cache statistics
    #[arg(long)]
    pub cache_stats: bool,
}

fn parse_pubkey(s: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(s).map_err(|e| format!("Invalid pubkey: {}", e))
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let secs: u64 = s.parse().map_err(|e| format!("Invalid timeout: {}", e))?;
    Ok(Duration::from_secs(secs))
}