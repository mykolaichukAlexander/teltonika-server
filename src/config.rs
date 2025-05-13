use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use log::{error, info};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThingsboardConfig {
    pub http_integration_url: String,
    pub auth_header_name: String,
    pub auth_header_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub thingsboard: ThingsboardConfig,
}

impl Config {
    pub fn load(config_path: &str) -> Result<Self, String> {
        let path = Path::new(config_path);
        
        if !path.exists() {
            error!("Configuration file not found at {}", config_path);
            return Err("Configuration file not found".to_string());
        }
        
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open configuration file: {}, using default configuration", e);
                return Err("Failed to open configuration file".to_string());
            }
        };
        
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            error!("Failed to read configuration file: {}, using default configuration", e);
            return Err("Failed to read configuration file".to_string());       
        }
        
        match serde_yaml::from_str(&contents) {
            Ok(config) => {
                info!("Configuration loaded successfully from {}", config_path);
                config
            },
            Err(e) => {
                error!("Failed to parse configuration file: {}, using default configuration", e);
                Err("Failed to parse configuration file".to_string())       
            }
        }
    }
}
