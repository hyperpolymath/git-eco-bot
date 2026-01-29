// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

//! # SustainaBot CLI
//!
//! Ecological and economic code analysis tool.
//! Built with Eclexia principles - proving resource-aware design works.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use sustainabot_analysis::analyze_file;
use tracing::info;

#[derive(Parser)]
#[command(name = "sustainabot")]
#[command(about = "Ecological & Economic Code Analysis", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a single file
    Analyze {
        /// File to analyze
        file: PathBuf,

        /// Output format (text, json, sarif)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Analyze a directory recursively
    Check {
        /// Directory to check
        path: PathBuf,

        /// Minimum eco score threshold (0-100)
        #[arg(long, default_value = "50")]
        eco_threshold: f64,
    },

    /// Show analysis of sustainabot itself (dogfooding!)
    SelfAnalyze,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

    match cli.command {
        Commands::Analyze { file, format } => {
            info!("Analyzing file: {}", file.display());
            let results = analyze_file(&file)?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&results)?;
                    println!("{}", json);
                }
                "text" => {
                    print_results_text(&results);
                }
                _ => {
                    eprintln!("Unsupported format: {}", format);
                }
            }
        }

        Commands::Check { path, eco_threshold } => {
            info!("Checking directory: {}", path.display());
            println!("Eco threshold: {}", eco_threshold);
            println!("(Recursive directory analysis coming soon)");
        }

        Commands::SelfAnalyze => {
            println!("🌱 SustainaBot Self-Analysis (Dogfooding!)");
            println!("==========================================\n");
            println!("Analyzing sustainabot's own resource usage...\n");

            // Analyze the analyzer!
            let analyzer_src = PathBuf::from("crates/sustainabot-analysis/src/analyzer.rs");
            if analyzer_src.exists() {
                let results = analyze_file(&analyzer_src)?;
                print_results_text(&results);

                println!("\n💡 Meta-Analysis:");
                println!("This analyzer used minimal resources to analyze itself.");
                println!("Eclexia-inspired design: explicit resource tracking from day 1.");
            } else {
                println!("Run from sustainabot repository root.");
            }
        }
    }

    Ok(())
}

fn print_results_text(results: &[sustainabot_metrics::AnalysisResult]) {
    for result in results {
        println!("\n📍 Function: {}", result.location.name.as_deref().unwrap_or("<anonymous>"));
        println!("   Location: {}:{}", result.location.line, result.location.column);
        println!("\n   Resources:");
        println!("     Energy:   {:.2} J", result.resources.energy.0);
        println!("     Time:     {:.2} ms", result.resources.duration.0);
        println!("     Carbon:   {:.4} gCO2e", result.resources.carbon.0);
        println!("     Memory:   {} bytes", result.resources.memory.0);

        println!("\n   Health Index:");
        println!("     Eco:      {:.1}/100", result.health.eco_score.0);
        println!("     Econ:     {:.1}/100", result.health.econ_score.0);
        println!("     Quality:  {:.1}/100", result.health.quality_score);
        println!("     Overall:  {:.1}/100", result.health.overall);

        if !result.recommendations.is_empty() {
            println!("\n   Recommendations:");
            for rec in &result.recommendations {
                println!("     • {}", rec);
            }
        }
    }

    println!("\n✅ Analysis complete");
}
