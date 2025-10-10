//! CLI for magneto-serge

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
use magneto_serge::{MatgtoProxy, ProxyMode};

#[cfg(feature = "cli")]
use std::path::{Path, PathBuf};

#[cfg(feature = "cli")]
use std::fs;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "matgto")]
#[command(version, about = "HTTP/WebSocket testing with record/replay", long_about = None)]
struct Cli {
    /// Cassette directory (default: ./cassettes)
    #[arg(short, long, global = true, default_value = "./cassettes")]
    cassette_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Start recording a new cassette
    Record {
        /// Cassette name
        name: String,

        /// Proxy port (default: 8888)
        #[arg(short, long, default_value = "8888")]
        port: u16,
    },

    /// Replay existing cassette
    Replay {
        /// Cassette name
        name: String,

        /// Proxy port (default: 8888)
        #[arg(short, long, default_value = "8888")]
        port: u16,
    },

    /// Start proxy in auto mode (record if cassette missing, else replay)
    Auto {
        /// Cassette name
        name: String,

        /// Proxy port (default: 8888)
        #[arg(short, long, default_value = "8888")]
        port: u16,
    },

    /// List available cassettes
    List,

    /// Show cassette details
    Inspect {
        /// Cassette name
        name: String,
    },

    /// Delete a cassette
    Delete {
        /// Cassette name
        name: String,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Initialize configuration
    Init,

    /// Show version information
    Version,
}

#[cfg(feature = "cli")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    // Ensure cassette directory exists
    if !cli.cassette_dir.exists() {
        fs::create_dir_all(&cli.cassette_dir)?;
        println!(
            "{} Created cassette directory: {}",
            "✓".green(),
            cli.cassette_dir.display().to_string().bold()
        );
    }

    match cli.command {
        Commands::Record { name, port } => {
            println!("{} {}", "⏺".red(), "Recording Mode".bold());
            println!("  Cassette: {}", name.bright_cyan());
            println!("  Port: {}", port.to_string().yellow());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!();
            println!(
                "Configure your HTTP client to use proxy: {}:{}",
                "localhost".green(),
                port.to_string().green()
            );
            println!("Press {} to stop recording...", "Ctrl+C".bold());
            println!();

            run_proxy(&cli.cassette_dir, &name, port, ProxyMode::Record)?;
        }

        Commands::Replay { name, port } => {
            println!("{} {}", "▶".green(), "Replay Mode".bold());
            println!("  Cassette: {}", name.bright_cyan());
            println!("  Port: {}", port.to_string().yellow());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!();
            println!(
                "Configure your HTTP client to use proxy: {}:{}",
                "localhost".green(),
                port.to_string().green()
            );
            println!("Press {} to stop...", "Ctrl+C".bold());
            println!();

            run_proxy(&cli.cassette_dir, &name, port, ProxyMode::Replay)?;
        }

        Commands::Auto { name, port } => {
            println!("{} {}", "🔄".blue(), "Auto Mode".bold());
            println!("  Cassette: {}", name.bright_cyan());
            println!("  Port: {}", port.to_string().yellow());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );

            let cassette_path = cli.cassette_dir.join(format!("{}.json", name));
            let mode = if cassette_path.exists() {
                println!("  Mode: {} (cassette exists)", "Replay".green());
                ProxyMode::Replay
            } else {
                println!("  Mode: {} (cassette missing)", "Record".red());
                ProxyMode::Record
            };

            println!();
            println!(
                "Configure your HTTP client to use proxy: {}:{}",
                "localhost".green(),
                port.to_string().green()
            );
            println!("Press {} to stop...", "Ctrl+C".bold());
            println!();

            run_proxy(&cli.cassette_dir, &name, port, mode)?;
        }

        Commands::List => {
            println!("{} {}", "📼".blue(), "Available Cassettes".bold());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!();

            list_cassettes(&cli.cassette_dir)?;
        }

        Commands::Inspect { name } => {
            println!("{} {}", "🔍".yellow(), "Cassette Details".bold());
            println!("  Name: {}", name.bright_cyan());
            println!();

            inspect_cassette(&cli.cassette_dir, &name)?;
        }

        Commands::Delete { name, yes } => {
            let cassette_path = cli.cassette_dir.join(format!("{}.json", name));

            if !cassette_path.exists() {
                println!("{} Cassette not found: {}", "✗".red(), name.bright_cyan());
                std::process::exit(1);
            }

            if !yes {
                println!("Delete cassette '{}'? [y/N] ", name.bright_cyan());
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("{} Cancelled", "✗".yellow());
                    return Ok(());
                }
            }

            fs::remove_file(&cassette_path)?;
            println!("{} Deleted cassette: {}", "✓".green(), name.bright_cyan());
        }

        Commands::Init => {
            println!(
                "{} {}",
                "🔧".yellow(),
                "Initializing matgto configuration...".bold()
            );

            let config_path = Path::new("matgto.toml");

            if config_path.exists() {
                println!(
                    "{} Configuration already exists: {}",
                    "✓".green(),
                    config_path.display().to_string().dimmed()
                );
                return Ok(());
            }

            let config = r#"# magneto-serge configuration

[proxy]
# Default proxy port
port = 8888

# Default mode: auto, record, replay, passthrough
mode = "auto"

[cassettes]
# Cassette storage directory
directory = "./cassettes"

# Cassette format: json or msgpack
format = "json"

[recording]
# Record request/response bodies
record_bodies = true

# Maximum body size to record (in bytes)
max_body_size = 1048576  # 1MB

[replay]
# Strict matching (fail if no match found)
strict = false

# Simulate network latency
simulate_latency = false
"#;

            fs::write(config_path, config)?;
            println!(
                "{} Created configuration: {}",
                "✓".green(),
                config_path.display().to_string().bright_cyan()
            );
        }

        Commands::Version => {
            println!("{}", "Magnéto-Serge".bold());
            println!("  Version: {}", env!("CARGO_PKG_VERSION").bright_cyan());
            println!("  Authors: {}", env!("CARGO_PKG_AUTHORS").dimmed());
            println!("  License: {}", env!("CARGO_PKG_LICENSE").dimmed());
        }
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn run_proxy(
    cassette_dir: &Path,
    cassette_name: &str,
    port: u16,
    mode: ProxyMode,
) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;
    use tokio::signal;

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        let proxy = MatgtoProxy::new_internal(cassette_dir)
            .map_err(|e| format!("Failed to create proxy: {}", e))?;

        proxy.set_port(port);
        proxy.set_mode(mode);

        match mode {
            ProxyMode::Record => {
                if !proxy.start_recording(cassette_name.to_string()) {
                    return Err("Failed to start recording".into());
                }
            }
            ProxyMode::Replay => {
                if !proxy.replay(cassette_name.to_string()) {
                    return Err("Failed to start replay".into());
                }
            }
            _ => {}
        }

        println!("{} Proxy running...", "✓".green());

        // Wait for Ctrl+C
        signal::ctrl_c().await?;

        println!();
        println!("{} Shutting down...", "⏹".yellow());

        if matches!(mode, ProxyMode::Record) && !proxy.stop_recording() {
            return Err("Failed to stop recording".into());
        }

        proxy.shutdown();

        println!("{} Done", "✓".green());

        Ok::<_, Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}

#[cfg(feature = "cli")]
fn list_cassettes(cassette_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;

    if !cassette_dir.exists() {
        println!("  {}", "No cassettes found".dimmed());
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(cassette_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "json" || s == "msgpack")
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    if entries.is_empty() {
        println!("  {}", "No cassettes found".dimmed());
        return Ok(());
    }

    for entry in entries {
        let path = entry.path();
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");

        let metadata = entry.metadata()?;
        let size = metadata.len();
        let size_kb = size as f64 / 1024.0;

        println!(
            "  {} {} {}",
            "•".bright_cyan(),
            name.bold(),
            format!("({:.1} KB)", size_kb).dimmed()
        );
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn inspect_cassette(cassette_dir: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;
    use magneto_serge::cassette::Cassette;

    let cassette_path = cassette_dir.join(format!("{}.json", name));

    if !cassette_path.exists() {
        println!("{} Cassette not found: {}", "✗".red(), name.bright_cyan());
        std::process::exit(1);
    }

    let content = fs::read_to_string(&cassette_path)?;
    let cassette: Cassette = serde_json::from_str(&content)?;

    println!("  Name: {}", cassette.name.bright_cyan());
    println!("  Version: {}", cassette.version.dimmed());
    println!("  Recorded: {}", cassette.recorded_at.to_string().dimmed());
    println!(
        "  Interactions: {}",
        cassette.interactions.len().to_string().yellow()
    );
    println!();

    if !cassette.interactions.is_empty() {
        println!("  {}", "Interactions:".bold());
        for (i, interaction) in cassette.interactions.iter().enumerate().take(10) {
            use magneto_serge::cassette::InteractionKind;
            match &interaction.kind {
                InteractionKind::Http { request, .. } => {
                    println!(
                        "    {}. {} {}",
                        (i + 1).to_string().dimmed(),
                        request.method.bright_green(),
                        request.url.dimmed()
                    );
                }
                InteractionKind::WebSocket { .. } => {
                    println!(
                        "    {}. {}",
                        (i + 1).to_string().dimmed(),
                        "WebSocket".bright_blue()
                    );
                }
            }
        }

        if cassette.interactions.len() > 10 {
            println!(
                "    {} ... and {} more",
                "•".dimmed(),
                (cassette.interactions.len() - 10).to_string().dimmed()
            );
        }
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI features not enabled. Build with --features cli");
    std::process::exit(1);
}
