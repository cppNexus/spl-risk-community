use spl_risk_core::model::report::RiskReport;
use anyhow::Result;
use colored::*;

pub fn print_report(report: &RiskReport, verbose: bool) -> Result<()> {
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════".bright_white());
    println!("{}", "SPL TOKEN RISK ANALYSIS".bright_white().bold());
    println!("{}", "═══════════════════════════════════════════════════════════".bright_white());
    println!();

    // Token & Profile
    println!("{}: {}", "TOKEN".bright_cyan().bold(), report.mint);
    println!("{}: {}", "PROFILE".bright_cyan().bold(), report.profile);
    println!();

    // RISK SCORE с цветом
    let risk_color = match report.risk_score {
        0..=20 => "green",
        21..=40 => "yellow",
        41..=60 => "bright_yellow",
        61..=80 => "red",
        _ => "bright_red",
    };

    println!(
        "{}: {}% [{}]",
        "RISK SCORE".bright_cyan().bold(),
        report.risk_score.to_string().color(risk_color).bold(),
        report.risk_level().color(risk_color).bold()
    );

    // CONFIDENCE с цветом
    let conf_color = match (report.confidence_score * 100.0) as u32 {
        90..=100 => "green",
        70..=89 => "yellow",
        _ => "red",
    };

    println!(
        "{}: {}% [{}]",
        "CONFIDENCE".bright_cyan().bold(),
        format!("{:.0}", report.confidence_score * 100.0).color(conf_color).bold(),
        report.confidence_level().color(conf_color).bold()
    );
    println!();

    // BREAKDOWN
    println!("{}", "BREAKDOWN:".bright_cyan().bold());
    println!("{}", "───────────────────────────────────────────────────────────".bright_black());

    for item in &report.breakdown {
        let weight_str = if item.weight >= 0 {
            format!("(+{})", item.weight).red()
        } else {
            format!("({})", item.weight).green()
        };

        let status_display = item.status.as_deref().unwrap_or("");
        let status_colored = match status_display {
            "verified"  => status_display.green().bold(),
            "revoked"   => status_display.green().bold(),
            "active"    => status_display.red().bold(),
            "retains"   => status_display.red().bold(),
            "unverified"=> status_display.yellow().bold(),
            "missing"   => status_display.yellow().bold(),
            "high"      => status_display.red().bold(),
            "low"       => status_display.yellow().bold(),
            "young"     => status_display.yellow().bold(),
            _           => status_display.white(),
        };

        println!(
            " ── {:<24} : {:<14} {:>7}  {}",
            item.rule.replace('_', " ").bright_white(),
            status_colored,
            weight_str,
            item.description.bright_black()
        );
    }
    println!();

    // METRICS (если verbose)
    if verbose {
        println!("{}", "METRICS:".bright_cyan().bold());
        println!("{}", "───────────────────────────────────────────────────────────".bright_black());

        // Все строки выровнены по 27 символам слева
        if let Some(supply) = report.metrics.total_supply {
            println!("  {:<27}: {}", "Total Supply", format_number(supply));
        }
        if let Some(d) = report.metrics.decimals {
            println!("  {:<27}: {}", "Decimals", d);
        }
        println!("  {:<27}: {:.2}%", "Creator Supply", report.metrics.creator_supply_pct);
        println!("  {:<27}: {}", "Holders", report.metrics.holders);

        if let Some(pct) = report.metrics.top_holder_pct {
            let warning = if pct > 30.0 { " ⚠ high concentration" } else { "" };
            println!("  {:<27}: {:.2}%{}", "Top Holder", pct, warning);
        }
        if let Some(age) = report.metrics.wallet_age_days {
            let years = age as f64 / 365.25;
            println!("  {:<27}: {} days ≈ {:.1} years", "Wallet Age", age, years);
        }

        println!();

        // DATA SOURCES
        println!("{}", "DATA SOURCES:".bright_cyan().bold());
        println!("{}", "───────────────────────────────────────────────────────────".bright_black());
        println!("  RPC          : {}", format_data_source(&report.data_sources.rpc));
        println!("  Metadata     : {}", format_data_source(&report.data_sources.metadata));
        println!("  Holders      : {}", format_data_source(&report.data_sources.holders));
        println!("  Wallet Age   : {}", format_data_source(&report.data_sources.wallet_age));

        if let Some(ref cached_at) = report.data_sources.cached_at {
            if !cached_at.is_empty() {
                println!();
                println!("  Cache timestamps:");
                for (key, ts) in cached_at {
                    println!("    {:<12} → {}", key, ts.bright_black());
                }
            }
        }
        println!();
    }

    // Edition Limitations (Community)
    #[cfg(not(feature = "lp-analysis"))]
    {
        println!("{}", "EDITION LIMITATIONS:".bright_white().bold());
        println!("{}", "───────────────────────────────────────────────────────────".bright_black());
        println!("  {} Liquidity pool analysis", "✗".red());
        println!("  {} LP lock / burn detection", "✗".red());
        println!("  {} Historical transaction patterns", "✗".red());
        println!("  {} Unlimited batch processing", "✗".red());
        println!();
    }

    // Warnings (если есть)
    if !report.warnings.is_empty() {
        println!("{}", "WARNINGS:".bright_yellow().bold());
        println!("{}", "───────────────────────────────────────────────────────────".bright_black());
        for warning in &report.warnings {
            println!("  !  {}", warning.yellow());
        }
        println!();
    }

    // SUMMARY — используем уже готовый report.summary + улучшение для low holders
    println!("{}", "SUMMARY:".bright_cyan().bold());
    println!("{}", "───────────────────────────────────────────────────────────".bright_black());

    let mut summary_text = report.summary.clone();

    // Дополняем summary, если holders мало (как в твоём примере)
    if report.risk_score <= 20 && report.metrics.holders < 50 {
        if !summary_text.contains("low holder count") {
            summary_text.push_str(
                " Main concern: very low holder count — possible early or concentrated project."
            );
        }
    }

    println!("  {}", summary_text);
    println!();

    // Disclaimer
    println!("{}", "DISCLAIMER:".bright_yellow().bold());
    println!("{}", "───────────────────────────────────────────────────────────".bright_black());
    println!("  Probabilistic assessment based on on-chain heuristics only.");
    println!("  NOT financial advice. Always DYOR (Do Your Own Research).");
    println!("{}", "═══════════════════════════════════════════════════════════".bright_white());
    println!();

    Ok(())
}

fn format_data_source(status: &str) -> colored::ColoredString {
    match status.to_lowercase().as_str() {
        "ok" => "✓ OK".green(),
        "cached" => "⚡ Cached".bright_blue(),
        "partial" => "⚠ Partial".yellow(),
        "timeout" => "⏱ Timeout".red(),
        "missing" => "✗ Missing".red(),
        "error" => "✗ Error".bright_red(),
        _ => status.normal(),
    }
}

fn format_number(n: u64) -> String {
        n.to_string()
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 3 == 0 {
                acc.push(',');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect()
}