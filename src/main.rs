mod models;
mod handlers;
mod services;
mod config;

use log::info;
use tokio::net::TcpListener;
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set default log level if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // Load configuration
    let config = Config::load("config.yaml")?;

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
