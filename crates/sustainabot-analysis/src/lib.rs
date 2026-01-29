// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

//! # SustainaBot Analysis Engine
//!
//! AST-based code analysis for ecological and economic metrics.
//! Built with Eclexia principles: explicit resource tracking.

pub mod analyzer;
pub mod carbon;
pub mod language;
pub mod patterns;

use anyhow::Result;
use sustainabot_metrics::AnalysisResult;
use std::path::Path;

pub use analyzer::Analyzer;
pub use language::Language;

/// Main entry point for analyzing a file
pub fn analyze_file(path: &Path) -> Result<Vec<AnalysisResult>> {
    let language = Language::detect(path)?;
    let mut analyzer = Analyzer::new(language)?;
    analyzer.analyze_file(path)
}

/// Analyze source code directly
pub fn analyze_source(source: &str, language: Language) -> Result<Vec<AnalysisResult>> {
    let mut analyzer = Analyzer::new(language)?;
    analyzer.analyze_source(source)
}
