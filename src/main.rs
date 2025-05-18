mod models;
mod handlers;
mod services;
mod config;

use log::{info, debug};
use tokio::net::TcpListener;
use crate::config::Config;
use clap::Parser;
use std::env;

/// Command line arguments for the Teltonika server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set default log level if RUST_LOG is not set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    
    // Initialize logger with stdout target for proper log separation
    env_logger::Builder::from_env(env_logger::Env::default())
        .target(env_logger::Target::Stdout) // Send logs to stdout instead of stderr
        .init();

    // Parse command line arguments
    let args = Args::parse();

    // Determine config path with the following priority:
    // 1. Command line argument
    // 2. Environment variable
    // 3. Default path
    let config_path = args.config
        .or_else(|| env::var("TELTONIKA_CONFIG_PATH").ok())
        .unwrap_or_else(|| "/etc/teltonika-server/config.json".to_string());

    debug!("Using configuration file: {}", config_path);

    // Load configuration
    let config = Config::load(&config_path)?;

    info!("Starting Teltonika server...");
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&bind_address).await?;
    info!("Server listening on {}", bind_address);
    info!("Thingsboard url will be user {}", config.thingsboard.http_integration_url);

    // Create a shared configuration for handlers
    let config_clone = config.clone();

    loop {
        let (socket, addr) = listener.accept().await?;
        // Clone the configuration for each connection handler
        let handler_config = config_clone.clone();

        tokio::spawn(async move {
            handlers::handle_connection(socket, addr, handler_config).await;
        });
    }
}
