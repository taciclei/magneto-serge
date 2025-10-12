//! CLI for magneto-serge

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
use magneto_serge::{ApiConfig, ApiServer, MagnetoProxy, ProxyMode};

#[cfg(feature = "cli")]
use std::path::{Path, PathBuf};

#[cfg(feature = "cli")]
use std::fs;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "magneto")]
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

    /// Clean old/unused cassettes
    Clean {
        /// Remove cassettes older than N days
        #[arg(short, long)]
        older_than: Option<u64>,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Validate cassette format
    Validate {
        /// Cassette name (optional, validates all if not specified)
        name: Option<String>,
    },

    /// Show or modify configuration
    Config {
        /// Configuration key to get/set
        key: Option<String>,

        /// Value to set (requires key)
        #[arg(short, long)]
        value: Option<String>,
    },

    /// Show version information
    Version,

    /// Start REST API server
    Api {
        /// API server host (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// API server port (default: 8889)
        #[arg(short, long, default_value = "8889")]
        port: u16,

        /// Proxy port (default: 8888)
        #[arg(long, default_value = "8888")]
        proxy_port: u16,

        /// Enable authentication
        #[arg(long)]
        auth: bool,

        /// API key for authentication (required if --auth is enabled)
        #[arg(long)]
        api_key: Option<String>,
    },
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
                "Initializing magneto configuration...".bold()
            );

            let config_path = Path::new("magneto.toml");

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

[api]
# Enable REST API server
enabled = false

# API server host
host = "127.0.0.1"

# API server port
port = 8889

# Enable authentication
auth_enabled = false

# API key (required if auth_enabled = true)
# api_key = "your-secret-key-here"
"#;

            fs::write(config_path, config)?;
            println!(
                "{} Created configuration: {}",
                "✓".green(),
                config_path.display().to_string().bright_cyan()
            );
        }

        Commands::Clean { older_than, yes } => {
            println!("{} {}", "🧹".yellow(), "Clean Cassettes".bold());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!();

            clean_cassettes(&cli.cassette_dir, older_than, yes)?;
        }

        Commands::Validate { name } => {
            println!("{} {}", "✓".green(), "Validate Cassettes".bold());
            println!(
                "  Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!();

            validate_cassettes(&cli.cassette_dir, name.as_deref())?;
        }

        Commands::Config { key, value } => {
            println!("{} {}", "⚙️".blue(), "Configuration".bold());
            println!();

            show_or_modify_config(key.as_deref(), value.as_deref())?;
        }

        Commands::Version => {
            println!("{}", "Magnéto-Serge".bold());
            println!("  Version: {}", env!("CARGO_PKG_VERSION").bright_cyan());
            println!("  Authors: {}", env!("CARGO_PKG_AUTHORS").dimmed());
            println!("  License: {}", env!("CARGO_PKG_LICENSE").dimmed());
        }

        Commands::Api {
            host,
            port,
            proxy_port,
            auth,
            api_key,
        } => {
            println!("{} {}", "🌐".blue(), "Starting API Server".bold());
            println!("  Host: {}", host.bright_cyan());
            println!("  Port: {}", port.to_string().yellow());
            println!("  Proxy Port: {}", proxy_port.to_string().yellow());
            println!(
                "  Cassette Directory: {}",
                cli.cassette_dir.display().to_string().dimmed()
            );
            println!(
                "  Authentication: {}",
                if auth {
                    "Enabled".green()
                } else {
                    "Disabled".dimmed()
                }
            );
            println!();

            if auth && api_key.is_none() {
                println!(
                    "{} API key required when authentication is enabled",
                    "✗".red()
                );
                println!("  Use {} to specify an API key", "--api-key <KEY>".bright_cyan());
                std::process::exit(1);
            }

            println!("{}", "API Endpoints:".bold());
            println!("  {} http://{}:{}/", "•".bright_cyan(), host, port);
            println!(
                "  {} http://{}:{}/openapi.json",
                "•".bright_cyan(),
                host,
                port
            );
            println!("  {} http://{}:{}/health", "•".bright_cyan(), host, port);
            println!("  {} http://{}:{}/proxy/*", "•".bright_cyan(), host, port);
            println!(
                "  {} http://{}:{}/cassettes",
                "•".bright_cyan(),
                host,
                port
            );
            println!();
            println!("Press {} to stop...", "Ctrl+C".bold());
            println!();

            run_api_server(&cli.cassette_dir, &host, port, proxy_port, auth, api_key)?;
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
        let proxy = MagnetoProxy::new_internal(cassette_dir)
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
                InteractionKind::HttpError { request, error } => {
                    println!(
                        "    {}. {} {} {}",
                        (i + 1).to_string().dimmed(),
                        request.method.bright_red(),
                        request.url.dimmed(),
                        format!("(Error: {:?})", error).red()
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

#[cfg(feature = "cli")]
fn clean_cassettes(
    cassette_dir: &Path,
    older_than: Option<u64>,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;
    use std::time::{SystemTime, UNIX_EPOCH};

    if !cassette_dir.exists() {
        println!("  {}", "No cassettes directory found".dimmed());
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(cassette_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "json" || s == "msgpack")
                .unwrap_or(false)
        })
        .collect();

    if entries.is_empty() {
        println!("  {}", "No cassettes found".dimmed());
        return Ok(());
    }

    let mut to_delete = Vec::new();

    if let Some(days) = older_than {
        let cutoff =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - (days * 24 * 60 * 60);

        for entry in entries {
            let metadata = entry.metadata()?;
            if let Ok(modified) = metadata.modified() {
                let modified_secs = modified.duration_since(UNIX_EPOCH)?.as_secs();
                if modified_secs < cutoff {
                    to_delete.push(entry.path());
                }
            }
        }
    } else {
        // Without --older-than, show what would be cleaned
        println!(
            "  {} Use {} to specify age threshold",
            "ℹ".blue(),
            "--older-than <days>".bold()
        );
        println!(
            "  Example: {} removes cassettes older than 30 days",
            "magneto clean --older-than 30".bright_cyan()
        );
        return Ok(());
    }

    if to_delete.is_empty() {
        println!("  {} No cassettes to clean", "✓".green());
        return Ok(());
    }

    println!(
        "  Found {} cassettes to delete:",
        to_delete.len().to_string().yellow()
    );
    for path in &to_delete {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
        println!("    {} {}", "•".red(), name);
    }
    println!();

    if !yes {
        println!("Delete {} cassettes? [y/N] ", to_delete.len());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Cancelled", "✗".yellow());
            return Ok(());
        }
    }

    let mut deleted = 0;
    for path in to_delete {
        if fs::remove_file(&path).is_ok() {
            deleted += 1;
        }
    }

    println!(
        "{} Deleted {} cassettes",
        "✓".green(),
        deleted.to_string().bold()
    );
    Ok(())
}

#[cfg(feature = "cli")]
fn validate_cassettes(
    cassette_dir: &Path,
    name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;
    use magneto_serge::cassette::Cassette;

    if !cassette_dir.exists() {
        println!("  {}", "No cassettes directory found".dimmed());
        return Ok(());
    }

    let entries: Vec<_> = if let Some(specific_name) = name {
        // Validate specific cassette
        let path = cassette_dir.join(format!("{}.json", specific_name));
        if !path.exists() {
            println!(
                "{} Cassette not found: {}",
                "✗".red(),
                specific_name.bright_cyan()
            );
            return Ok(());
        }
        vec![path]
    } else {
        // Validate all cassettes
        fs::read_dir(cassette_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == "json" || s == "msgpack")
                    .unwrap_or(false)
            })
            .map(|e| e.path())
            .collect()
    };

    if entries.is_empty() {
        println!("  {}", "No cassettes found".dimmed());
        return Ok(());
    }

    let mut valid = 0;
    let mut invalid = 0;

    for path in entries {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<Cassette>(&content) {
                Ok(cassette) => {
                    println!(
                        "  {} {} ({} interactions)",
                        "✓".green(),
                        name.bold(),
                        cassette.interactions.len().to_string().dimmed()
                    );
                    valid += 1;
                }
                Err(e) => {
                    println!(
                        "  {} {} - {}",
                        "✗".red(),
                        name.bold(),
                        format!("{}", e).dimmed()
                    );
                    invalid += 1;
                }
            },
            Err(e) => {
                println!(
                    "  {} {} - {}",
                    "✗".red(),
                    name.bold(),
                    format!("{}", e).dimmed()
                );
                invalid += 1;
            }
        }
    }

    println!();
    if invalid == 0 {
        println!(
            "{} All {} cassettes are valid",
            "✓".green(),
            valid.to_string().bold()
        );
    } else {
        println!(
            "{} {} valid, {} invalid",
            "⚠".yellow(),
            valid.to_string().green(),
            invalid.to_string().red()
        );
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn show_or_modify_config(
    key: Option<&str>,
    value: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;

    let config_path = Path::new("magneto.toml");

    if !config_path.exists() {
        println!("{} Configuration file not found", "✗".red());
        println!("  Run {} to create it", "magneto init".bright_cyan());
        return Ok(());
    }

    if key.is_none() && value.is_none() {
        // Show entire config
        let content = fs::read_to_string(config_path)?;
        println!("  File: {}", config_path.display().to_string().dimmed());
        println!();
        for line in content.lines() {
            if line.trim().starts_with('#') {
                println!("  {}", line.dimmed());
            } else if line.contains('=') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    println!(
                        "  {} = {}",
                        parts[0].trim().bright_cyan(),
                        parts[1].trim().yellow()
                    );
                } else {
                    println!("  {}", line);
                }
            } else if line.trim().starts_with('[') {
                println!("  {}", line.bold());
            } else {
                println!("  {}", line);
            }
        }
        return Ok(());
    }

    if let Some(k) = key {
        let content = fs::read_to_string(config_path)?;

        if let Some(v) = value {
            // Set value
            let mut new_content = String::new();
            let mut found = false;

            for line in content.lines() {
                if line.trim().starts_with(&format!("{} =", k))
                    || line.trim().starts_with(&format!("{}=", k))
                {
                    new_content.push_str(&format!("{} = {}\n", k, v));
                    found = true;
                } else {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }

            if !found {
                println!("{} Key not found: {}", "✗".red(), k.bright_cyan());
                return Ok(());
            }

            fs::write(config_path, new_content)?;
            println!("{} Set {} = {}", "✓".green(), k.bright_cyan(), v.yellow());
        } else {
            // Get value
            for line in content.lines() {
                if line.trim().starts_with(&format!("{} =", k))
                    || line.trim().starts_with(&format!("{}=", k))
                {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        println!("  {} = {}", k.bright_cyan(), parts[1].trim().yellow());
                        return Ok(());
                    }
                }
            }
            println!("{} Key not found: {}", "✗".red(), k.bright_cyan());
        }
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn run_api_server(
    cassette_dir: &Path,
    host: &str,
    port: u16,
    proxy_port: u16,
    auth: bool,
    api_key: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        let config = ApiConfig {
            host: host.to_string(),
            port,
            proxy_port,
            cassette_dir: cassette_dir.display().to_string(),
            auth_enabled: auth,
            api_key,
        };

        let server = ApiServer::new(config);

        println!("{} API Server running...", "✓".green());

        // Start the server (this blocks until Ctrl+C)
        if let Err(e) = server.start().await {
            println!("{} Server error: {}", "✗".red(), e);
            return Err(e.into());
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    })?;

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI features not enabled. Build with --features cli");
    std::process::exit(1);
}
