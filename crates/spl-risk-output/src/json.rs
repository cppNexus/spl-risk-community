use spl_risk_core::model::RiskReport;
use anyhow::Result;

pub fn print_report(report: &RiskReport) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    println!("{}", json);
    Ok(())
}