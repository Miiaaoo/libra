use clap::{Args, Subcommand};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::utils::error::CliError;
use crate::utils::output::OutputConfig;

#[derive(Debug, Args)]
pub struct StatsArgs {
    #[arg(long)]
    pub json_output: bool,
}

#[derive(Debug, Subcommand)]
pub enum StatsCmd {
    #[command(about = "统计当前目录中的文件分布")]
    Run(StatsArgs),
}

pub async fn execute_safe(cmd: &StatsCmd, _output: &OutputConfig) -> Result<(), CliError> {
    match cmd {
        StatsCmd::Run(stats) => execute_stats(stats).await,
    }
}

async fn execute_stats(stats: &StatsArgs) -> Result<(), CliError> {
    let current_dir = std::env::current_dir()
        .map_err(|e| CliError::fatal(e.to_string()))?;
    let stats_data = collect_stats(&current_dir)
        .map_err(|e| CliError::fatal(e.to_string()))?;

    if stats.json_output {
        let json = serde_json::to_string_pretty(&stats_data)
            .map_err(|e| CliError::fatal(e.to_string()))?;
        println!("{}", json);
    } else {
        println!("Total files: {}", stats_data.total_files);
        println!("\nExtension distribution:");
        for (ext, count) in &stats_data.extensions {
            println!("  {:15} : {}", ext, count);
        }
    }
    Ok(())
}

fn collect_stats(dir: &Path) -> Result<StatsOutput, CliError> {
    let mut extensions = BTreeMap::new();
    let mut total_files = 0;

    fn visit(
        dir: &Path,
        extensions: &mut BTreeMap<String, usize>,
        total: &mut usize,
    ) -> Result<(), CliError> {
        for entry in fs::read_dir(dir).map_err(|e| CliError::fatal(e.to_string()))? {
            let entry = entry.map_err(|e| CliError::fatal(e.to_string()))?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == ".libra" || name == "target" {
                    continue;
                }
                visit(&path, extensions, total)?;
            } else if path.is_file() {
                *total += 1;
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "no_extension".to_string());
                *extensions.entry(ext).or_insert(0) += 1;
            }
        }
        Ok(())
    }

    visit(dir, &mut extensions, &mut total_files)?;
    Ok(StatsOutput {
        extensions,
        total_files,
    })
}

#[derive(Debug, Serialize)]
pub struct StatsOutput {
    pub extensions: BTreeMap<String, usize>,
    pub total_files: usize,
}