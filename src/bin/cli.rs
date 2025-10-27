//! Magnéto-Serge CLI Tool
//!
//! Command-line interface for managing HTTP/WebSocket test cassettes.
//!
//! ## Commands
//!
//! - `list`     - List all cassettes
//! - `validate` - Validate cassette integrity
//! - `clean`    - Remove old/large cassettes
//! - `stats`    - Show cassette statistics

#![allow(clippy::too_many_arguments)]
//! - `export`   - Export cassettes to different formats
//! - `serve`    - Start REST API server
//! - `migrate`  - Migrate cassettes between versions
//! - `replay`   - Replay mode (use cassettes without recording)
//! - `record`   - Record mode (capture new interactions)
//! - `init`     - Initialize magneto.toml configuration

use clap::{Parser, Subcommand};
use colored::*;
#[cfg(not(feature = "hydra"))]
use magneto_serge::api::handlers::start_server;
#[cfg(feature = "hydra")]
use magneto_serge::api::handlers::start_server_with_hydra;
use magneto_serge::{api::cassettes::CassetteManager, error::Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "magneto")]
#[command(about = "Magnéto-Serge - HTTP/WebSocket testing tool", long_about = None)]
#[command(version, author)]
struct Cli {
    /// Cassette directory path
    #[arg(short, long, default_value = "./cassettes")]
    cassette_dir: PathBuf,

    /// Output format: text, json, table
    #[arg(short = 'f', long, default_value = "table")]
    format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
    Table,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "table" => Ok(OutputFormat::Table),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// List all cassettes
    List {
        /// Sort by: name, size, age, interactions
        #[arg(short, long, default_value = "name")]
        sort_by: String,

        /// Sort order: asc, desc
        #[arg(short, long, default_value = "asc")]
        order: String,

        /// Filter by minimum age in days
        #[arg(long)]
        min_age_days: Option<i64>,

        /// Filter by maximum age in days
        #[arg(long)]
        max_age_days: Option<i64>,

        /// Filter by minimum size in bytes
        #[arg(long)]
        min_size_bytes: Option<u64>,

        /// Filter by maximum size in bytes
        #[arg(long)]
        max_size_bytes: Option<u64>,
    },

    /// Validate cassette integrity
    Validate {
        /// Cassette name (without extension), or "all" for all cassettes
        name: String,

        /// Show only errors (hide warnings)
        #[arg(short, long)]
        errors_only: bool,
    },

    /// Clean up old or large cassettes
    Clean {
        /// Remove cassettes older than N days
        #[arg(long)]
        older_than_days: Option<i64>,

        /// Remove cassettes larger than N MB
        #[arg(long)]
        larger_than_mb: Option<u64>,

        /// Dry run (show what would be deleted without deleting)
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Show cassette statistics
    Stats {
        /// Cassette name (without extension), or "all" for global stats
        name: String,
    },

    /// Export cassettes to different formats
    Export {
        /// Cassette name (without extension)
        name: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Export format: json, msgpack, yaml, har
        #[arg(short = 'f', long, default_value = "json")]
        format: String,
    },

    /// Start REST API server
    Serve {
        /// Server host
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,

        /// Server port
        #[arg(short, long, default_value = "8889")]
        port: u16,
    },

    /// Migrate cassettes between versions
    Migrate {
        /// Source version (e.g., "1.0")
        #[arg(short, long)]
        from: String,

        /// Target version (e.g., "2.0")
        #[arg(short, long)]
        to: String,

        /// Cassette name (without extension), or "all" for all cassettes
        name: String,

        /// Backup original cassettes before migration
        #[arg(short, long)]
        backup: bool,
    },

    /// Replay mode (use cassettes without recording)
    Replay {
        /// Proxy port
        #[arg(short, long, default_value = "8888")]
        port: u16,

        /// Strict mode (error on missing interactions)
        #[arg(short, long)]
        strict: bool,
    },

    /// Record mode (capture new interactions)
    Record {
        /// Cassette name (without extension)
        name: String,

        /// Proxy port
        #[arg(short, long, default_value = "8888")]
        port: u16,

        /// Enable filtering
        #[arg(short, long)]
        filter: bool,

        /// Overwrite existing cassette
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Initialize magneto.toml configuration
    Init {
        /// Overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging if verbose
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    let manager = CassetteManager::new(&cli.cassette_dir);

    match cli.command {
        Commands::List {
            sort_by,
            order,
            min_age_days,
            max_age_days,
            min_size_bytes,
            max_size_bytes,
        } => {
            cmd_list(
                &manager,
                &cli.format,
                &sort_by,
                &order,
                min_age_days,
                max_age_days,
                min_size_bytes,
                max_size_bytes,
            )?;
        }

        Commands::Validate { name, errors_only } => {
            cmd_validate(&manager, &name, errors_only)?;
        }

        Commands::Clean {
            older_than_days,
            larger_than_mb,
            dry_run,
            force,
        } => {
            cmd_clean(&manager, older_than_days, larger_than_mb, dry_run, force)?;
        }

        Commands::Stats { name } => {
            cmd_stats(&manager, &name, &cli.format)?;
        }

        Commands::Export {
            name,
            output,
            format,
        } => {
            cmd_export(&manager, &name, &output, &format)?;
        }

        Commands::Serve { host, port } => {
            cmd_serve(&host, port, &cli.cassette_dir).await?;
        }

        Commands::Migrate {
            from,
            to,
            name,
            backup,
        } => {
            cmd_migrate(&manager, &from, &to, &name, backup)?;
        }

        Commands::Replay { port, strict } => {
            cmd_replay(port, strict, &cli.cassette_dir)?;
        }

        Commands::Record {
            name,
            port,
            filter,
            overwrite,
        } => {
            cmd_record(&name, port, filter, overwrite, &cli.cassette_dir)?;
        }

        Commands::Init { force } => {
            cmd_init(force)?;
        }
    }

    Ok(())
}

// ============================================================
// COMMAND IMPLEMENTATIONS
// ============================================================

/// List cassettes
fn cmd_list(
    manager: &CassetteManager,
    format: &OutputFormat,
    sort_by: &str,
    order: &str,
    min_age_days: Option<i64>,
    max_age_days: Option<i64>,
    min_size_bytes: Option<u64>,
    max_size_bytes: Option<u64>,
) -> Result<()> {
    let mut cassettes = manager.list_cassettes()?;

    // Apply filters
    if let Some(min_age) = min_age_days {
        cassettes.retain(|c| c.age_days >= min_age);
    }
    if let Some(max_age) = max_age_days {
        cassettes.retain(|c| c.age_days <= max_age);
    }
    if let Some(min_size) = min_size_bytes {
        cassettes.retain(|c| c.size_bytes >= min_size);
    }
    if let Some(max_size) = max_size_bytes {
        cassettes.retain(|c| c.size_bytes <= max_size);
    }

    // Sort
    match sort_by {
        "size" => cassettes.sort_by_key(|c| c.size_bytes),
        "age" => cassettes.sort_by_key(|c| c.age_days),
        "interactions" => cassettes.sort_by_key(|c| c.interaction_count),
        _ => cassettes.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    if order == "desc" {
        cassettes.reverse();
    }

    // Output
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&cassettes)?);
        }
        OutputFormat::Text => {
            for cassette in cassettes {
                println!(
                    "{} - {} - {} interactions - {} days old",
                    cassette.name,
                    cassette.size_human,
                    cassette.interaction_count,
                    cassette.age_days
                );
            }
        }
        OutputFormat::Table => {
            println!("\n{}\n", "📼 Cassettes".bright_cyan().bold());
            println!(
                "{:<40} {:>12} {:>15} {:>10}",
                "Name".bold(),
                "Size".bold(),
                "Interactions".bold(),
                "Age".bold()
            );
            println!("{}", "─".repeat(80).bright_black());

            for cassette in cassettes {
                let age_color = if cassette.age_days > 90 {
                    "red"
                } else if cassette.age_days > 30 {
                    "yellow"
                } else {
                    "green"
                };

                println!(
                    "{:<40} {:>12} {:>15} {:>10}",
                    cassette.name.bright_white(),
                    cassette.size_human.bright_blue(),
                    cassette.interaction_count.to_string().bright_green(),
                    format!("{} days", cassette.age_days).color(age_color)
                );
            }
            println!();
        }
    }

    Ok(())
}

/// Validate cassette(s)
fn cmd_validate(manager: &CassetteManager, name: &str, errors_only: bool) -> Result<()> {
    if name == "all" {
        let cassettes = manager.list_cassettes()?;
        let mut total_errors = 0;
        let mut total_warnings = 0;

        println!(
            "\n{}\n",
            "🔍 Validating all cassettes...".bright_cyan().bold()
        );

        for cassette in &cassettes {
            let result = manager.validate_cassette(&cassette.name)?;

            if !result.valid || (!errors_only && !result.warnings.is_empty()) {
                println!("{}", format!("📼 {}", cassette.name).bright_white().bold());

                if !result.errors.is_empty() {
                    for error in &result.errors {
                        println!("  {} {}", "❌".red(), error);
                    }
                    total_errors += result.errors.len();
                }

                if !errors_only && !result.warnings.is_empty() {
                    for warning in &result.warnings {
                        println!("  {} {}", "⚠️ ".yellow(), warning);
                    }
                    total_warnings += result.warnings.len();
                }

                println!();
            }
        }

        println!("{} {} cassettes validated", "✅".green(), cassettes.len());
        if total_errors > 0 {
            println!("{} {} errors found", "❌".red(), total_errors);
        }
        if total_warnings > 0 && !errors_only {
            println!("{} {} warnings found", "⚠️ ".yellow(), total_warnings);
        }
    } else {
        let result = manager.validate_cassette(name)?;

        println!(
            "\n{} {}\n",
            "🔍 Validating cassette:".bright_cyan().bold(),
            name.bright_white()
        );

        if result.valid && result.warnings.is_empty() {
            println!("{} Cassette is valid", "✅".green());
        } else {
            if !result.errors.is_empty() {
                println!("{}", "Errors:".red().bold());
                for error in &result.errors {
                    println!("  {} {}", "❌".red(), error);
                }
            }

            if !errors_only && !result.warnings.is_empty() {
                println!("\n{}", "Warnings:".yellow().bold());
                for warning in &result.warnings {
                    println!("  {} {}", "⚠️ ".yellow(), warning);
                }
            }
        }
        println!();
    }

    Ok(())
}

/// Clean up cassettes
fn cmd_clean(
    manager: &CassetteManager,
    older_than_days: Option<i64>,
    larger_than_mb: Option<u64>,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    let cassettes = manager.list_cassettes()?;
    let mut to_delete = Vec::new();

    for cassette in cassettes {
        let mut should_delete = false;

        if let Some(max_age) = older_than_days {
            if cassette.age_days > max_age {
                should_delete = true;
            }
        }

        if let Some(max_size_mb) = larger_than_mb {
            let size_mb = cassette.size_bytes / (1024 * 1024);
            if size_mb > max_size_mb {
                should_delete = true;
            }
        }

        if should_delete {
            to_delete.push(cassette);
        }
    }

    if to_delete.is_empty() {
        println!("{} No cassettes match cleanup criteria", "ℹ️ ".blue());
        return Ok(());
    }

    println!("\n{}\n", "🗑️  Cassettes to delete:".bright_yellow().bold());

    let mut total_size = 0u64;
    for cassette in &to_delete {
        total_size += cassette.size_bytes;
        println!(
            "  {} {} ({}, {} days old)",
            "•".bright_yellow(),
            cassette.name.bright_white(),
            cassette.size_human.bright_blue(),
            cassette.age_days
        );
    }

    println!(
        "\n{} {} cassettes, {} total",
        "📊".bright_cyan(),
        to_delete.len(),
        format_bytes(total_size).bright_blue()
    );

    if dry_run {
        println!("\n{} Dry run - no files deleted", "ℹ️ ".blue());
        return Ok(());
    }

    if !force {
        print!("\n{} ", "❓ Delete these cassettes? [y/N]:".bright_yellow());
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let mut deleted = 0;
    for cassette in &to_delete {
        match manager.delete_cassette(&cassette.name) {
            Ok(_) => {
                println!("  {} Deleted {}", "✅".green(), cassette.name);
                deleted += 1;
            }
            Err(e) => {
                println!("  {} Failed to delete {}: {}", "❌".red(), cassette.name, e);
            }
        }
    }

    println!("\n{} {} cassettes deleted", "✅".green(), deleted);

    Ok(())
}

/// Show statistics
fn cmd_stats(manager: &CassetteManager, name: &str, format: &OutputFormat) -> Result<()> {
    if name == "all" {
        let stats = manager.global_stats()?;

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            }
            _ => {
                println!("\n{}\n", "📊 Global Statistics".bright_cyan().bold());
                println!(
                    "Total Cassettes: {}",
                    stats.total_count.to_string().bright_white()
                );
                println!("Total Size: {}", stats.total_size_human.bright_blue());

                if let Some(oldest) = &stats.oldest_cassette {
                    println!("\nOldest Cassette:");
                    println!("  Name: {}", oldest.name.bright_white());
                    println!(
                        "  Age: {} days",
                        oldest.age_days.to_string().bright_yellow()
                    );
                }

                if let Some(largest) = &stats.largest_cassette {
                    println!("\nLargest Cassette:");
                    println!("  Name: {}", largest.name.bright_white());
                    println!("  Size: {}", largest.size_human.bright_blue());
                }

                println!("\nSize Distribution:");
                println!(
                    "  < 1 MB: {}",
                    stats.size_distribution.under_1mb.to_string().bright_green()
                );
                println!(
                    "  1-10 MB: {}",
                    stats
                        .size_distribution
                        .mb_1_to_10
                        .to_string()
                        .bright_yellow()
                );
                println!(
                    "  > 10 MB: {}",
                    stats.size_distribution.over_10mb.to_string().bright_red()
                );

                println!("\nAge Distribution:");
                println!(
                    "  < 7 days: {}",
                    stats
                        .age_distribution
                        .under_7_days
                        .to_string()
                        .bright_green()
                );
                println!(
                    "  7-30 days: {}",
                    stats
                        .age_distribution
                        .days_7_to_30
                        .to_string()
                        .bright_yellow()
                );
                println!(
                    "  > 30 days: {}",
                    stats.age_distribution.over_30_days.to_string().bright_red()
                );
                println!();
            }
        }
    } else {
        let stats = manager.get_cassette_stats(name)?;

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            }
            _ => {
                println!(
                    "\n{} {}\n",
                    "📊 Cassette Statistics:".bright_cyan().bold(),
                    name.bright_white()
                );
                println!(
                    "Total Interactions: {}",
                    stats.total_interactions.to_string().bright_white()
                );
                println!(
                    "HTTP Requests: {}",
                    stats.http_count.to_string().bright_green()
                );
                println!(
                    "WebSocket Messages: {}",
                    stats.websocket_count.to_string().bright_blue()
                );
                println!(
                    "HTTP Errors: {}",
                    stats.http_error_count.to_string().bright_red()
                );

                println!("\nHTTP Methods:");
                for (method, count) in &stats.http_methods {
                    println!(
                        "  {}: {}",
                        method.bright_white(),
                        count.to_string().bright_cyan()
                    );
                }

                println!("\nStatus Codes:");
                for (code, count) in &stats.status_codes {
                    let color = if *code >= 200 && *code < 300 {
                        "green"
                    } else if *code >= 400 {
                        "red"
                    } else {
                        "yellow"
                    };
                    println!(
                        "  {}: {}",
                        code.to_string().color(color),
                        count.to_string().bright_cyan()
                    );
                }

                println!("\nBody Sizes:");
                println!(
                    "  Request: {}",
                    format_bytes(stats.total_request_body_bytes as u64).bright_blue()
                );
                println!(
                    "  Response: {}",
                    format_bytes(stats.total_response_body_bytes as u64).bright_blue()
                );

                if let Some(avg_time) = stats.avg_response_time_ms {
                    println!("\nAverage Response Time: {:.2} ms", avg_time);
                }
                println!();
            }
        }
    }

    Ok(())
}

/// Export cassette
fn cmd_export(manager: &CassetteManager, name: &str, output: &PathBuf, format: &str) -> Result<()> {
    println!(
        "\n{} Exporting cassette '{}' to {:?}...\n",
        "📦".bright_cyan(),
        name.bright_white(),
        output
    );

    let cassette = manager.load_cassette(name)?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&cassette)?;
            std::fs::write(output, json)?;
        }
        "yaml" => {
            println!(
                "{} YAML export not yet implemented (TODO: Phase 2)",
                "⚠️ ".yellow()
            );
            return Ok(());
        }
        "har" => {
            println!(
                "{} HAR export not yet implemented (TODO: Phase 2)",
                "⚠️ ".yellow()
            );
            return Ok(());
        }
        "msgpack" => {
            println!(
                "{} MessagePack export not yet implemented (TODO: Phase 2)",
                "⚠️ ".yellow()
            );
            return Ok(());
        }
        _ => {
            return Err(magneto_serge::error::MatgtoError::CassetteLoadFailed {
                reason: format!("Unknown export format: {}", format),
            });
        }
    }

    println!("{} Exported successfully to {:?}", "✅".green(), output);

    Ok(())
}

/// Start API server
async fn cmd_serve(host: &str, port: u16, cassette_dir: &PathBuf) -> Result<()> {
    println!(
        "\n{}",
        "🚀 Starting Magnéto-Serge API Server..."
            .bright_cyan()
            .bold()
    );
    println!("📂 Cassette directory: {:?}", cassette_dir);
    println!("🌐 Listening on: {}:{}", host, port);

    #[cfg(feature = "hydra")]
    {
        println!("📖 REST API: http://{}:{}/cassettes", host, port);
        println!("📖 Hydra API: http://{}:{}/api/cassettes\n", host, port);
        println!("{} Press Ctrl+C to stop\n", "ℹ️ ".blue());

        // Use Hydra-enabled server for full hypermedia support
        start_server_with_hydra(host, port, cassette_dir).await?;
    }

    #[cfg(not(feature = "hydra"))]
    {
        println!("📖 API documentation: http://{}:{}/health\n", host, port);
        println!("{} Press Ctrl+C to stop\n", "ℹ️ ".blue());

        // Use REST API only
        start_server(host, port, cassette_dir).await?;
    }

    Ok(())
}

/// Migrate cassettes
fn cmd_migrate(
    _manager: &CassetteManager,
    from: &str,
    to: &str,
    _name: &str,
    _backup: bool,
) -> Result<()> {
    println!(
        "\n{} Migrating cassettes from v{} to v{}\n",
        "🔄".bright_cyan(),
        from,
        to
    );

    if from == "1.0" && to == "2.0" {
        // TODO: Implement v1.0 → v2.0 migration
        println!(
            "{} v1.0 → v2.0 migration not yet implemented",
            "⚠️ ".yellow()
        );
        println!("   This will:");
        println!("   - Add cookies field");
        println!("   - Add filtered metadata field");
        println!("   - Update version number");
    } else {
        println!("{} Unsupported migration: {} → {}", "❌".red(), from, to);
    }

    Ok(())
}

/// Replay mode
fn cmd_replay(port: u16, strict: bool, cassette_dir: &PathBuf) -> Result<()> {
    println!(
        "\n{}",
        "▶️  Starting Magnéto-Serge in REPLAY mode"
            .bright_green()
            .bold()
    );
    println!("📂 Cassette directory: {:?}", cassette_dir);
    println!("🌐 Proxy port: {}", port);
    println!(
        "📏 Strict mode: {}",
        if strict {
            "enabled".bright_red()
        } else {
            "disabled".bright_green()
        }
    );
    println!(
        "\n{} Configure your app to use proxy: http://localhost:{}\n",
        "ℹ️ ".blue(),
        port
    );
    println!(
        "{} Replay mode not yet fully implemented (TODO: integrate with proxy)\n",
        "⚠️ ".yellow()
    );

    Ok(())
}

/// Record mode
fn cmd_record(
    name: &str,
    port: u16,
    filter: bool,
    overwrite: bool,
    cassette_dir: &PathBuf,
) -> Result<()> {
    println!(
        "\n{}",
        "⏺️  Starting Magnéto-Serge in RECORD mode"
            .bright_red()
            .bold()
    );
    println!("📼 Cassette name: {}", name.bright_white());
    println!("📂 Cassette directory: {:?}", cassette_dir);
    println!("🌐 Proxy port: {}", port);
    println!(
        "🔍 Filtering: {}",
        if filter {
            "enabled".bright_green()
        } else {
            "disabled".bright_yellow()
        }
    );
    println!(
        "📝 Overwrite: {}",
        if overwrite {
            "yes".bright_red()
        } else {
            "no".bright_green()
        }
    );
    println!(
        "\n{} Configure your app to use proxy: http://localhost:{}\n",
        "ℹ️ ".blue(),
        port
    );
    println!(
        "{} Record mode not yet fully implemented (TODO: integrate with proxy)\n",
        "⚠️ ".yellow()
    );

    Ok(())
}

/// Initialize configuration
fn cmd_init(force: bool) -> Result<()> {
    let config_path = std::path::Path::new("magneto.toml");

    if config_path.exists() && !force {
        println!(
            "{} magneto.toml already exists. Use --force to overwrite.",
            "⚠️ ".yellow()
        );
        return Ok(());
    }

    let default_config = r#"# Magneto-Serge Configuration
#
# This file configures the HTTP/WebSocket proxy behavior.

[magneto]
# Directory where cassettes are stored
cassette_dir = "./cassettes"

# Default proxy port
proxy_port = 8888

# Default mode: auto, record, replay, passthrough
mode = "auto"

# Strict mode for replay (error if interaction not found)
strict = true

[matching]
# Headers to ignore when matching requests
ignore_headers = ["User-Agent", "Date", "X-Request-Id", "Accept-Encoding"]

# Query parameters to ignore
ignore_query_params = ["timestamp", "_t", "cache_bust"]

[recording]
# Headers to filter from cassettes (sensitive data)
filter_headers = ["Authorization", "X-API-Key", "Cookie", "Set-Cookie"]

# Compress cassettes with gzip
compress = false

# Format: json or msgpack
format = "json"

[filters]
# Enable smart filtering to reduce cassette size
enabled = true

# Preset: web_assets, api_only, minimal, or custom
preset = "web_assets"

# Custom extensions to exclude (if preset = "custom")
exclude_extensions = [".js", ".css", ".png", ".jpg", ".woff2", ".svg"]

# Status codes to exclude
exclude_status_codes = [404, 500, 502, 503]

[api]
# REST API server configuration
enabled = false
host = "127.0.0.1"
port = 8889
auth_enabled = false
"#;
    std::fs::write(config_path, default_config)?;

    println!(
        "\n{} Created magneto.toml with default configuration",
        "✅".green()
    );
    println!("\n{} Edit magneto.toml to customize:", "ℹ️ ".blue());
    println!("  - Cassette directory");
    println!("  - Proxy port");
    println!("  - Filtering rules");
    println!("  - Cookie preservation");
    println!("  - API server settings\n");

    Ok(())
}

// ============================================================
// HELPERS
// ============================================================

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
