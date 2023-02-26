use anyhow::anyhow;
use serde::Deserialize;
use std::net::IpAddr;
use tracing::info;

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct MagnetarNetworking {
    pub host: String,
    pub port: u16,
    pub bind_addr: IpAddr,
}

fn env_host() -> String {
    std::env::var("MAG_C_HOST")
        .expect("MAG_C_HOST or \"host\" in the default configuration must be set")
}

fn env_bind_addr() -> IpAddr {
    std::env::var("MAG_C_BIND_ADDR")
        .unwrap_or_else(|_| "::".to_owned())
        .parse()
        .map_err(|e| format!("Failed to parse \"MAG_C_BIND_ADDR\": {e}"))
        .unwrap()
}

fn env_port() -> u16 {
    std::env::var("MAG_C_PORT")
        .unwrap_or_else(|_| "4939".to_owned())
        .parse()
        .expect("MAG_C_PORT must be a valid port number")
}

impl Default for MagnetarNetworking {
    fn default() -> Self {
        MagnetarNetworking {
            host: env_host(),
            bind_addr: env_bind_addr(),
            port: env_port(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[non_exhaustive]
pub struct MagnetarConfig {
    #[serde(default)]
    pub networking: MagnetarNetworking,
}

pub fn load_config() -> anyhow::Result<MagnetarConfig> {
    let path =
        std::env::var("MAG_CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_owned());

    let str_cfg =
        std::fs::read_to_string(path).map_err(|e| anyhow!("Failed to load configuration: {e}"))?;

    let config =
        toml::from_str(&str_cfg).map_err(|e| anyhow!("Failed to parse configuration: {e}"))?;

    info!("Loaded configuration: {config:#?}");

    Ok(config)
}
